#include <algorithm>
#include <array>
#include <chrono>
#include <cmath>
#include <cstdint>
#include <functional>
#include <limits>
#include <memory>
#include <mutex>
#include <stdexcept>
#include <string>

#include "nav_msgs/msg/odometry.hpp"
#include "rclcpp/rclcpp.hpp"
#include "std_msgs/msg/bool.hpp"
#include "std_srvs/srv/trigger.hpp"
#include "tf2/LinearMath/Matrix3x3.h"
#include "tf2/LinearMath/Quaternion.h"
#include "tf2/LinearMath/Vector3.h"

// 匿名命令空间
namespace
{
// 定义终端 ANSI 彩色字体转义码
constexpr const char * kColorReset   = "\033[0m";
constexpr const char * kColorRed     = "\033[31m";
constexpr const char * kColorGreen   = "\033[32m";
constexpr const char * kColorYellow  = "\033[33m";
constexpr const char * kColorBlue    = "\033[34m";
constexpr const char * kColorMagenta = "\033[35m"; // 紫红色
constexpr const char * kColorCyan    = "\033[36m"; // 青色
constexpr const char * kColorBold    = "\033[1m";  // 高亮粗体
constexpr const char * kColorBoldGreen = "\033[1;32m"; // 高亮粗体绿

constexpr double kPi = 3.14159265358979323846;

// &：引用传递
bool finiteVector(const geometry_msgs::msg::Vector3 & v)
{
  return finite(v.x) && finite(v.y) && finite(v.z);
}

bool finitePoint(const geometry_msgs::msg::Point & p)
{
  return finite(p.x) && finite(p.y) && finite(p.z);
}

bool finiteQuaternion(const geometry_msgs::msg::Quaternion & q)
{
  return finite(q.x) && finite(q.y) && finite(q.z) && finite(q.w);
}

tf2::Vector3 toTf(const geometry_msgs::msg::Point & p)
{
  return {p.x, p.y, p.z};
}

tf2::Vector3 toTf(const geometry_msgs::msg::Vector3 & v)
{
  return {v.x, v.y, v.z};
}

tf2::Quaternion toTf(const geometry_msgs::msg::Quaternion & q)
{
  return {q.x, q.y, q.z, q.w};
}

geometry_msgs::msg::Point toPoint(const tf2::Vector3 & v)
{
  geometry_msgs::msg::Point result;
  result.x = v.x();
  result.y = v.y();
  result.z = v.z();
  return result;
}

geometry_msgs::msg::Vector3 toVector(const tf2::Vector3 & v)
{
  geometry_msgs::msg::Vector3 result;
  result.x = v.x();
  result.y = v.y();
  result.z = v.z();
  return result;
}

geometry_msgs::msg::Quaternion toQuaternion(const tf2::Quaternion & q)
{
  geometry_msgs::msg::Quaternion result;
  result.x = q.x();
  result.y = q.y();
  result.z = q.z();
  result.w = q.w();
  return result;
}

template<typename Covariance>
bool covarianceIsZero(const Covariance & covariance)
{
  for (const double value : covariance) {
    if (!finite(value) || std::abs(value) > 1.0e-15) {
      return false;
    }
  }
  return true;
}

template<typename Covariance>
bool covarianceIsFinite(const Covariance & covariance)
{
  for (const double value : covariance) {
    if (!finite(value)) {
      return false;
    }
  }
  return true;
}

template<typename Covariance>
Covariance rotateCovariance(
  const Covariance & input, const tf2::Matrix3x3 & linear_rotation,
  const tf2::Matrix3x3 & angular_rotation)
{
  Covariance output{};
  double a[6][6]{};
  for (int row = 0; row < 3; ++row) {
    for (int col = 0; col < 3; ++col) {
      a[row][col] = linear_rotation[row][col];
      a[row + 3][col + 3] = angular_rotation[row][col];
    }
  }

  for (int row = 0; row < 6; ++row) {
    for (int col = 0; col < 6; ++col) {
      double value = 0.0;
      for (int i = 0; i < 6; ++i) {
        for (int j = 0; j < 6; ++j) {
          value += a[row][i] * input[i * 6 + j] * a[col][j];
        }
      }
      output[row * 6 + col] = value;
    }
  }
  return output;
}

template<typename Covariance>
Covariance diagonalCovariance(double linear_stddev, double angular_stddev)
{
  Covariance covariance{};
  const double linear_variance = linear_stddev * linear_stddev;
  const double angular_variance = angular_stddev * angular_stddev;
  covariance[0] = linear_variance;
  covariance[7] = linear_variance;
  covariance[14] = linear_variance;
  covariance[21] = angular_variance;
  covariance[28] = angular_variance;
  covariance[35] = angular_variance;
  return covariance;
}
}  // namespace

class VisionToMavros : public rclcpp::Node
{
public:
  VisionToMavros()
  : Node("vision_to_mavros")
  {
    input_topic_ = declare_parameter<std::string>("input_topic", "/vins_estimator/odometry");
    output_topic_ = declare_parameter<std::string>("output_topic", "/mavros/odometry/in");
    output_frame_id_ = declare_parameter<std::string>("output_frame_id", "map");
    output_child_frame_id_ = declare_parameter<std::string>("output_child_frame_id", "base_link");
    yaw_offset_rad_ = declare_parameter<double>("yaw_offset_deg", 0.0) * kPi / 180.0;
    align_initial_yaw_ = declare_parameter<bool>("align_initial_yaw", false);
    zero_position_ = declare_parameter<bool>("zero_position", true);
    linear_velocity_frame_ =
      declare_parameter<std::string>("input_linear_velocity_frame", "world");
    angular_velocity_frame_ =
      declare_parameter<std::string>("input_angular_velocity_frame", "body");

    // Safety & Fault Tolerance
    max_input_age_s_ = declare_parameter<double>("max_input_age_s", 0.25);
    max_message_gap_s_ = declare_parameter<double>("max_message_gap_s", 0.50);
    max_position_jump_m_ = declare_parameter<double>("max_position_jump_m", 1.0);
    max_orientation_jump_rad_ =
      declare_parameter<double>("max_orientation_jump_deg", 60.0) * kPi / 180.0;
    reject_zero_stamp_ = declare_parameter<bool>("reject_zero_stamp", true);
    latch_faults_ = declare_parameter<bool>("latch_faults", true);

    // Covariance & Stddevs
    fill_covariance_if_zero_ = declare_parameter<bool>("fill_covariance_if_zero", true);
    position_stddev_m_ = declare_parameter<double>("position_stddev_m", 0.10);
    orientation_stddev_rad_ = declare_parameter<double>("orientation_stddev_rad", 0.10);
    linear_velocity_stddev_mps_ =
      declare_parameter<double>("linear_velocity_stddev_mps", 0.20);
    angular_velocity_stddev_radps_ =
      declare_parameter<double>("angular_velocity_stddev_radps", 0.10);

    if (linear_velocity_frame_ != "world" && linear_velocity_frame_ != "body") {
      throw std::invalid_argument("input_linear_velocity_frame must be 'world' or 'body'");
    }
    if (angular_velocity_frame_ != "world" && angular_velocity_frame_ != "body") {
      throw std::invalid_argument("input_angular_velocity_frame must be 'world' or 'body'");
    }

    auto output_qos = rclcpp::QoS(rclcpp::KeepLast(2)).reliable().durability_volatile();
    odometry_publisher_ = create_publisher<nav_msgs::msg::Odometry>(output_topic_, output_qos);
    valid_publisher_ = create_publisher<std_msgs::msg::Bool>(
      "~/valid", rclcpp::QoS(1).reliable().transient_local());

    odometry_subscription_ = create_subscription<nav_msgs::msg::Odometry>(
      input_topic_, rclcpp::SensorDataQoS().keep_last(5),
      std::bind(&VisionToMavros::odometryCallback, this, std::placeholders::_1));

    // Reset Service
    reset_service_ = create_service<std_srvs::srv::Trigger>(
      "~/reset",
      std::bind(
        &VisionToMavros::resetCallback, this, std::placeholders::_1,
        std::placeholders::_2));

    watchdog_timer_ = create_wall_timer(
      std::chrono::milliseconds(100), std::bind(&VisionToMavros::watchdogCallback, this));

    publishValidity(false);
    RCLCPP_INFO(
      get_logger(), "%s Bridging %s%s -> %s%s (local Z-up/ENU-compatible, MAVROS performs NED conversion) %s",
      kColorBoldGreen, kColorYellow, input_topic_.c_str(), output_topic_.c_str(), kColorReset, kColorReset);
  }

private:
  void reject(const std::string & reason, bool latch = true)
  {
    if (latch && latch_faults_) {
      fault_latched_ = true;
    }
    publishValidity(false);
    RCLCPP_ERROR_THROTTLE(get_logger(), *get_clock(), 2000, "Odometry rejected: %s", reason.c_str());
  }

  void publishValidity(bool valid)
  {
    if (validity_known_ && last_validity_ == valid) {
      return;
    }
    std_msgs::msg::Bool message;
    message.data = valid;
    valid_publisher_->publish(message);
    validity_known_ = true;
    last_validity_ = valid;
  }

  // 位置与姿态的重置与转换
  // Position and orientation reset and transformation
  void initializeReference(const nav_msgs::msg::Odometry & input, const tf2::Quaternion & q_vb)
  {
    origin_v_ = zero_position_ ? toTf(input.pose.pose.position) : tf2::Vector3(0.0, 0.0, 0.0);

    double initial_roll = 0.0;
    double initial_pitch = 0.0;
    double initial_yaw = 0.0;
    tf2::Matrix3x3(q_vb).getRPY(initial_roll, initial_pitch, initial_yaw);
    const double alignment_yaw = yaw_offset_rad_ - (align_initial_yaw_ ? initial_yaw : 0.0);
    q_ev_.setRPY(0.0, 0.0, alignment_yaw);
    q_ev_.normalize();
    r_ev_ = tf2::Matrix3x3(q_ev_);
    initialized_ = true;

    RCLCPP_INFO(
      get_logger(),
      "Origin initialized: origin=[%.3f %.3f %.3f], yaw offset: %s%.3f deg %s",
      origin_v_.x(), origin_v_.y(), origin_v_.z(),
      kColorMagenta, alignment_yaw * 180.0 / kPi, kColorReset ); 

  }

  void odometryCallback(const nav_msgs::msg::Odometry::SharedPtr input)
  {
    std::lock_guard<std::mutex> lock(mutex_);
    const auto receive_time = now();

    if (fault_latched_) {
      publishValidity(false);
      return;
    }

    const int64_t stamp_ns = rclcpp::Time(input->header.stamp).nanoseconds();
    if (reject_zero_stamp_ && stamp_ns == 0) {
      reject("zero timestamp");
      return;
    }
    if (have_previous_input_ && stamp_ns <= previous_stamp_ns_) {
      reject("timestamp did not increase");
      return;
    }

    if (max_input_age_s_ > 0.0 && stamp_ns != 0) {
      const rclcpp::Time stamp(input->header.stamp, get_clock()->get_clock_type());
      const double age = (receive_time - stamp).seconds();
      if (age > max_input_age_s_ || age < -0.10) {
        reject("timestamp age is outside allowed range");
        return;
      }
    }

    if (!finitePoint(input->pose.pose.position) ||
      !finiteQuaternion(input->pose.pose.orientation) ||
      !finiteVector(input->twist.twist.linear) ||
      !finiteVector(input->twist.twist.angular) ||
      !covarianceIsFinite(input->pose.covariance) ||
      !covarianceIsFinite(input->twist.covariance))
    {
      reject("NaN or Inf in pose/twist");
      return;
    }

    tf2::Quaternion q_vb = toTf(input->pose.pose.orientation);
    const double quaternion_norm = q_vb.length();
    if (!finite(quaternion_norm) || quaternion_norm < 1.0e-6) {
      reject("invalid orientation quaternion");
      return;
    }
    q_vb.normalize();
    const tf2::Vector3 p_v = toTf(input->pose.pose.position);

    if (have_previous_input_) {
      const double dt = static_cast<double>(stamp_ns - previous_stamp_ns_) * 1.0e-9;
      if (max_message_gap_s_ > 0.0 && dt > max_message_gap_s_) {
        reject("input message gap exceeded limit");
        return;
      }
      if (max_position_jump_m_ > 0.0 && (p_v - previous_position_v_).length() > max_position_jump_m_) {
        reject("position jump exceeded limit");
        return;
      }
      double quaternion_dot = std::abs(q_vb.dot(previous_orientation_vb_));
      quaternion_dot = std::max(-1.0, std::min(1.0, quaternion_dot));
      const double angular_jump = 2.0 * std::acos(quaternion_dot);
      if (max_orientation_jump_rad_ > 0.0 && angular_jump > max_orientation_jump_rad_) {
        reject("orientation jump exceeded limit");
        return;
      }
    }

    if (!initialized_) {
      initializeReference(*input, q_vb);
    }

    nav_msgs::msg::Odometry output;
    output.header.stamp = input->header.stamp;  // Preserve measurement time for EKF delay compensation.
    output.header.frame_id = output_frame_id_;
    output.child_frame_id = output_child_frame_id_;

    output.pose.pose.position = toPoint(r_ev_ * (p_v - origin_v_));
    tf2::Quaternion q_eb = q_ev_ * q_vb;
    q_eb.normalize();
    output.pose.pose.orientation = toQuaternion(q_eb);

    const tf2::Matrix3x3 r_vb(q_vb);
    const tf2::Matrix3x3 r_bv = r_vb.transpose();
    const tf2::Matrix3x3 identity(tf2::Quaternion::getIdentity());

    // nav_msgs/Odometry defines twist in child_frame_id (body FLU here).
    const tf2::Matrix3x3 linear_to_body =
      linear_velocity_frame_ == "world" ? r_bv : identity;
    const tf2::Matrix3x3 angular_to_body =
      angular_velocity_frame_ == "world" ? r_bv : identity;
    output.twist.twist.linear = toVector(linear_to_body * toTf(input->twist.twist.linear));
    output.twist.twist.angular = toVector(angular_to_body * toTf(input->twist.twist.angular));

    if (fill_covariance_if_zero_ && covarianceIsZero(input->pose.covariance)) {
      output.pose.covariance = diagonalCovariance<decltype(output.pose.covariance)>(
        position_stddev_m_, orientation_stddev_rad_);
    } else {
      output.pose.covariance = rotateCovariance(input->pose.covariance, r_ev_, r_ev_);
    }

    if (fill_covariance_if_zero_ && covarianceIsZero(input->twist.covariance)) {
      output.twist.covariance = diagonalCovariance<decltype(output.twist.covariance)>(
        linear_velocity_stddev_mps_, angular_velocity_stddev_radps_);
    } else {
      output.twist.covariance = rotateCovariance(
        input->twist.covariance, linear_to_body, angular_to_body);
    }

    odometry_publisher_->publish(output);
    previous_stamp_ns_ = stamp_ns;
    previous_position_v_ = p_v;
    previous_orientation_vb_ = q_vb;
    last_valid_wall_time_ = std::chrono::steady_clock::now();
    have_previous_input_ = true;
    have_valid_receive_time_ = true;
    publishValidity(true);
  }

  void watchdogCallback()
  {
    std::lock_guard<std::mutex> lock(mutex_);
    if (!have_valid_receive_time_ || max_input_age_s_ <= 0.0) {
      return;
    }
    const double silence_s = std::chrono::duration<double>(
      std::chrono::steady_clock::now() - last_valid_wall_time_).count();
    if (silence_s > max_input_age_s_) {
      if (latch_faults_) {
        fault_latched_ = true;
      }
      publishValidity(false);
      RCLCPP_ERROR_THROTTLE(
        get_logger(), *get_clock(), 2000,
        "VINS odometry is stale; output stopped%s",
        latch_faults_ ? " and fault latched" : "");
    }
  }

  void resetCallback(
    const std::shared_ptr<std_srvs::srv::Trigger::Request>,
    std::shared_ptr<std_srvs::srv::Trigger::Response> response)
  {
    std::lock_guard<std::mutex> lock(mutex_);
    initialized_ = false;
    have_previous_input_ = false;
    have_valid_receive_time_ = false;
    fault_latched_ = false;
    previous_stamp_ns_ = 0;
    publishValidity(false);
    response->success = true;
    response->message = "Bridge fault and local reference cleared; waiting for fresh VINS odometry";
    RCLCPP_WARN(get_logger(), "Bridge manually reset; the next valid pose becomes the new reference");
  }

  std::mutex mutex_;
  std::string input_topic_;
  std::string output_topic_;
  std::string output_frame_id_;
  std::string output_child_frame_id_;
  std::string linear_velocity_frame_;
  std::string angular_velocity_frame_;
  double yaw_offset_rad_{0.0};
  double max_input_age_s_{0.25};
  double max_message_gap_s_{0.50};
  double max_position_jump_m_{1.0};
  double max_orientation_jump_rad_{kPi / 3.0};
  double position_stddev_m_{0.10};
  double orientation_stddev_rad_{0.10};
  double linear_velocity_stddev_mps_{0.20};
  double angular_velocity_stddev_radps_{0.10};
  bool align_initial_yaw_{false};
  bool zero_position_{true};
  bool reject_zero_stamp_{true};
  bool latch_faults_{true};
  bool fill_covariance_if_zero_{true};
  bool initialized_{false};
  bool have_previous_input_{false};
  bool have_valid_receive_time_{false};
  bool fault_latched_{false};
  bool validity_known_{false};
  bool last_validity_{false};
  int64_t previous_stamp_ns_{0};
  tf2::Vector3 origin_v_{0.0, 0.0, 0.0};
  tf2::Vector3 previous_position_v_{0.0, 0.0, 0.0};
  tf2::Quaternion previous_orientation_vb_{0.0, 0.0, 0.0, 1.0};
  tf2::Quaternion q_ev_{0.0, 0.0, 0.0, 1.0};
  tf2::Matrix3x3 r_ev_{tf2::Quaternion::getIdentity()};
  std::chrono::steady_clock::time_point last_valid_wall_time_{};

  rclcpp::Subscription<nav_msgs::msg::Odometry>::SharedPtr odometry_subscription_;
  rclcpp::Publisher<nav_msgs::msg::Odometry>::SharedPtr odometry_publisher_;
  rclcpp::Publisher<std_msgs::msg::Bool>::SharedPtr valid_publisher_;
  rclcpp::Service<std_srvs::srv::Trigger>::SharedPtr reset_service_;
  rclcpp::TimerBase::SharedPtr watchdog_timer_;
};

int main(int argc, char ** argv)
{
  rclcpp::init(argc, argv);
  try {
    rclcpp::spin(std::make_shared<VisionToMavros>());
  } catch (const std::exception & exception) {
    RCLCPP_FATAL(rclcpp::get_logger("vision_to_mavros"), "%s", exception.what());
    rclcpp::shutdown();
    return 1;
  }
  rclcpp::shutdown();
  return 0;
}
