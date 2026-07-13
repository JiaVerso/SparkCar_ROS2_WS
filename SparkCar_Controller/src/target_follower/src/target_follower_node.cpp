/*
# @file target_follower_node.cpp
# @author JiaVerso
# @brief  A C++ ROS 2 node designed to follow a detected target
          Uses the geometry messages to control the robot's movement
# @version 0.1
# @date 2026-07-12
#
# @copyright JiaVerso (c) 2026
*/

#include <algorithm>
#include <chrono>
#include <cmath>
#include <functional>
#include <memory>
#include <mutex>
#include <stdexcept>
#include <string>

#include <geometry_msgs/msg/point_stamped.hpp>
#include <geometry_msgs/msg/twist.hpp>
#include <rclcpp/rclcpp.hpp>
#include <std_srvs/srv/set_bool.hpp>

#include "target_follower/pid_controller.hpp"

using namespace std::chrono_literals;

namespace target_follower
{

class TargetFollowerNode : public rclcpp::Node
{
public:
  TargetFollowerNode()
  : Node("target_follower_node")
  {
    // declare topics 
    target_topic_ = declare_parameter<std::string>(
      "target_topic", "/yolo/target_point_3d");
    cmd_vel_topic_ = declare_parameter<std::string>("cmd_vel_topic", "/cmd_vel");

    // custom parameters
    desired_distance_ = declare_parameter<double>("desired_distance", 1.2);
    target_timeout_ = declare_parameter<double>("target_timeout", 0.5);
    control_frequency_ = declare_parameter<double>("control_frequency", 20.0);

    // PID controller parameters
    distance_kp_ = declare_parameter<double>("distance_kp", 0.7);
    heading_kp_ = declare_parameter<double>("heading_kp", 1.2);
    distance_deadband_ = declare_parameter<double>("distance_deadband", 0.10);
    heading_deadband_ = declare_parameter<double>("heading_deadband", 0.025);
    filter_alpha_ = declare_parameter<double>("filter_alpha", 0.35);

    // Motion constraints parameters
    max_forward_speed_ = declare_parameter<double>("max_forward_speed", 0.45);
    max_reverse_speed_ = declare_parameter<double>("max_reverse_speed", 0.12);
    min_tracking_speed_ = declare_parameter<double>("min_tracking_speed", 0.05);
    max_angular_speed_ = declare_parameter<double>("max_angular_speed", 0.25);
    max_linear_accel_ = declare_parameter<double>("max_linear_accel", 0.45);
    max_linear_decel_ = declare_parameter<double>("max_linear_decel", 0.8);
    max_angular_accel_ = declare_parameter<double>("max_angular_accel", 0.6);
    minimum_turning_radius_ = declare_parameter<double>("minimum_turning_radius", 1.8);
    heading_slowdown_angle_ = declare_parameter<double>("heading_slowdown_angle", 0.6);
    min_heading_speed_scale_ = declare_parameter<double>("min_heading_speed_scale", 0.25);

    min_valid_depth_ = declare_parameter<double>("min_valid_depth", 0.2);
    max_valid_depth_ = declare_parameter<double>("max_valid_depth", 6.0);

    // 急停距离
    emergency_stop_distance_ = declare_parameter<double>("emergency_stop_distance", 0.6);
    allow_reverse_ = declare_parameter<bool>("allow_reverse", false);
    start_enabled_ = declare_parameter<bool>("start_enabled", false);

    // 参数检查
    validate_parameters();

    // initialize
    distance_pid_.configure(
      distance_kp_, 0.0, 0.0, 0.0, 0.0,
      std::max(max_forward_speed_, max_reverse_speed_));
    heading_pid_.configure(
      heading_kp_, 0.0, 0.0, 0.0, 0.0, max_angular_speed_);
    enabled_ = start_enabled_;

    cmd_vel_publisher_ =
      create_publisher<geometry_msgs::msg::Twist>(cmd_vel_topic_, rclcpp::QoS(10));

    target_subscription_ = create_subscription<geometry_msgs::msg::PointStamped>(
      target_topic_, rclcpp::QoS(10),
      std::bind(&TargetFollowerNode::target_callback, this, std::placeholders::_1));

    // 创建服务
    enable_service_ = create_service<std_srvs::srv::SetBool>(
      "~/set_enabled",
      std::bind(
        &TargetFollowerNode::enable_callback, this,
        std::placeholders::_1, std::placeholders::_2));

    // 定时器
    last_control_time_ = now();
    const auto period = std::chrono::duration<double>(1.0 / control_frequency_);
    control_timer_ = create_wall_timer(
      std::chrono::duration_cast<std::chrono::nanoseconds>(period),
      std::bind(&TargetFollowerNode::control_callback, this));

    publish_stop();

    RCLCPP_INFO(
      get_logger(),
      "Target follower ready (PID interface, P-only): target=%s, output=%s, "
      "desired_distance=%.2fm, enabled=%s",
      target_topic_.c_str(), cmd_vel_topic_.c_str(), desired_distance_,
      enabled_ ? "true" : "false");
    RCLCPP_WARN(
      get_logger(), "Do not run Nav2 controller output on %s at the same time.",
      cmd_vel_topic_.c_str());
  }

  ~TargetFollowerNode() override
  {
    publish_stop();
  }

private:
  void validate_parameters() const
  {
    if (desired_distance_ <= 0.0 || target_timeout_ <= 0.0 || control_frequency_ <= 0.0) {
      throw std::runtime_error("distance, timeout and control frequency must be positive");
    }
    if (distance_kp_ < 0.0 || heading_kp_ < 0.0 ||
      max_forward_speed_ <= 0.0 || max_angular_speed_ <= 0.0)
    {
      throw std::runtime_error("controller gains and speed limits are invalid");
    }
    if (filter_alpha_ <= 0.0 || filter_alpha_ > 1.0 ||
      min_heading_speed_scale_ < 0.0 || min_heading_speed_scale_ > 1.0)
    {
      throw std::runtime_error("filter_alpha and speed scale must be within (0, 1]");
    }
    if (minimum_turning_radius_ <= 0.0 || max_linear_accel_ <= 0.0 ||
      max_linear_decel_ <= 0.0 ||
      max_angular_accel_ <= 0.0 || heading_slowdown_angle_ <= 0.0)
    {
      throw std::runtime_error("motion constraint parameters must be positive");
    }
    if (min_valid_depth_ < 0.0 || max_valid_depth_ <= min_valid_depth_) {
      throw std::runtime_error("valid depth range is invalid");
    }
    if (emergency_stop_distance_ <= min_valid_depth_ ||
      emergency_stop_distance_ >= desired_distance_)
    {
      throw std::runtime_error("emergency_stop_distance must be below desired_distance");
    }
  }

  // 低通滤波后 point.x / point.z
  void target_callback(const geometry_msgs::msg::PointStamped::SharedPtr message)
  {
    const double right = message->point.x;
    const double forward = message->point.z;

    // 判断传入值
    if (!std::isfinite(right) || !std::isfinite(forward) ||
      forward < min_valid_depth_ || forward > max_valid_depth_)
    {
      RCLCPP_WARN_THROTTLE(
        get_logger(), *get_clock(), 2000,
        "Ignoring invalid target point: right=%.3f, forward=%.3f", right, forward);
      return;
    }

    // add pid caulculate mutex lock
    std::lock_guard<std::mutex> lock(target_mutex_);

    // 一阶低通滤波
    if (!has_target_) {
      filtered_right_ = right;
      filtered_forward_ = forward;
    } else {
      filtered_right_ = filter_alpha_ * right + (1.0 - filter_alpha_) * filtered_right_;
      filtered_forward_ =
        filter_alpha_ * forward + (1.0 - filter_alpha_) * filtered_forward_;
    }
    last_target_time_ = now();
    has_target_ = true;
  }

  // service 开启/关闭目标跟随
  void enable_callback(
    const std_srvs::srv::SetBool::Request::SharedPtr request,
    std_srvs::srv::SetBool::Response::SharedPtr response)
  {
    {
      std::lock_guard<std::mutex> lock(target_mutex_);
      enabled_ = request->data;
      if (!enabled_) {
        has_target_ = false;
      }
    }
    if (!request->data) {
      publish_stop();
    }
    response->success = true;
    response->message = request->data ? "target following enabled" : "target following disabled";
    RCLCPP_INFO(get_logger(), "%s", response->message.c_str());
  }

  // 
  static double rate_limit(double desired, double previous, double maximum_step)
  {
    return previous + std::clamp(desired - previous, -maximum_step, maximum_step);
  }

  void control_callback()
  {
    const rclcpp::Time current_time = now();
    bool enabled = false;
    bool has_target = false;
    double target_right = 0.0;
    double target_forward = 0.0;
    rclcpp::Time target_time(0, 0, get_clock()->get_clock_type());
    {
      std::lock_guard<std::mutex> lock(target_mutex_);
      enabled = enabled_;
      has_target = has_target_;
      target_right = filtered_right_;
      target_forward = filtered_forward_;
      target_time = last_target_time_;
    }

    // 检查是否启用了目标跟随
    if (!enabled) {
      publish_stop_if_needed();
      last_control_time_ = current_time;
      return;
    }

    // 目标丢失/超时拦截
    const double target_age = has_target ? (current_time - target_time).seconds() : 1e9;
    if (!has_target || target_age > target_timeout_) {
      publish_stop_if_needed();
      RCLCPP_WARN_THROTTLE(
        get_logger(), *get_clock(), 2000,
        "Target lost or stale (age=%.2fs), stopping", target_age);
      last_control_time_ = current_time;
      return;
    }

    // 急停距离判断
    if (target_forward <= emergency_stop_distance_) {
      publish_stop_if_needed();
      RCLCPP_WARN_THROTTLE(
        get_logger(), *get_clock(), 1000,
        "Target is inside emergency stop distance: %.2fm <= %.2fm",
        target_forward, emergency_stop_distance_);
      last_control_time_ = current_time;
      return;
    }

    // 误差计算
    const double distance_error = target_forward - desired_distance_;
    const double heading_error = std::atan2(-target_right, target_forward);
    
    // 根据纵向和横向距离算偏向角
    const double distance_control = -distance_pid_.calculate(
      target_forward, desired_distance_);
    const double heading_control = -heading_pid_.calculate(heading_error, 0.0);

    // 输出限幅
    double desired_linear = 0.0;
    if (distance_error > distance_deadband_) {
      desired_linear = std::clamp(
        distance_control, min_tracking_speed_, max_forward_speed_);
    } else if (distance_error < -distance_deadband_ && allow_reverse_) {
      desired_linear = std::clamp(
        distance_control, -max_reverse_speed_, -min_tracking_speed_);
    }

    // 急弯减速
    // 急弯过大，默认最小25%速度
    if (desired_linear != 0.0) {
      const double speed_scale = std::max(
        min_heading_speed_scale_,
        1.0 - std::abs(heading_error) / heading_slowdown_angle_);
      desired_linear *= std::clamp(speed_scale, min_heading_speed_scale_, 1.0);
    }

    double desired_angular = 0.0;
    if (desired_linear != 0.0 && std::abs(heading_error) > heading_deadband_) {
      
      // 阿克曼角度约束
      const double ackermann_limit =
        std::abs(desired_linear) / minimum_turning_radius_;
      const double angular_limit = std::min(max_angular_speed_, ackermann_limit);
      desired_angular = std::clamp(
        heading_control, -angular_limit, angular_limit);
    }

    double dt = (current_time - last_control_time_).seconds();
    dt = std::clamp(dt, 0.001, 0.2);

    // 判断减速
    const bool decelerating =
      std::abs(desired_linear) < std::abs(previous_linear_) ||
      desired_linear * previous_linear_ < 0.0;

    // 加速度或减速度
    const double linear_rate = decelerating ? max_linear_decel_ : max_linear_accel_;

    // 创建 ROS 速度消息
    geometry_msgs::msg::Twist command;

    command.linear.x = rate_limit(
      desired_linear, previous_linear_, linear_rate * dt);
    command.angular.z = rate_limit(
      desired_angular, previous_angular_, max_angular_accel_ * dt);

    // 角速度 ω = 线速度 v / 转弯半径 R
    const double final_angular_limit = std::min(
      max_angular_speed_, std::abs(command.linear.x) / minimum_turning_radius_);
    command.angular.z = std::clamp(
      command.angular.z, -final_angular_limit, final_angular_limit);

    cmd_vel_publisher_->publish(command);
    previous_linear_ = command.linear.x;
    previous_angular_ = command.angular.z;
    last_control_time_ = current_time;

    RCLCPP_INFO_THROTTLE(
      get_logger(), *get_clock(), 1000,
      "target: right=%.2fm forward=%.2fm, error: distance=%.2fm heading=%.1fdeg, cmd: v=%.2f w=%.2f",
      target_right, target_forward, distance_error, heading_error * 180.0 / M_PI,
      command.linear.x, command.angular.z);
  }

  void publish_stop_if_needed()
  {
    if (previous_linear_ != 0.0 || previous_angular_ != 0.0) {
      publish_stop();
    }
  }

  void publish_stop()
  {
    if (cmd_vel_publisher_) {

      // 右值引用
      cmd_vel_publisher_->publish(geometry_msgs::msg::Twist());
    }
    previous_linear_ = 0.0;
    previous_angular_ = 0.0;
    distance_pid_.reset();
    heading_pid_.reset();
  }

  std::string target_topic_;
  std::string cmd_vel_topic_;
  double desired_distance_{1.2};
  double target_timeout_{0.5};
  double control_frequency_{20.0};
  double distance_kp_{0.7};
  double heading_kp_{1.2};
  double distance_deadband_{0.1};
  double heading_deadband_{0.025};
  double filter_alpha_{0.35};
  double max_forward_speed_{0.45};
  double max_reverse_speed_{0.12};
  double min_tracking_speed_{0.05};
  double max_angular_speed_{0.25};
  double max_linear_accel_{0.45};
  double max_linear_decel_{0.8};
  double max_angular_accel_{0.6};
  double minimum_turning_radius_{1.8};
  double heading_slowdown_angle_{0.6};
  double min_heading_speed_scale_{0.25};
  double min_valid_depth_{0.2};
  double max_valid_depth_{6.0};
  double emergency_stop_distance_{0.6};
  bool allow_reverse_{false};
  bool start_enabled_{false};

  std::mutex target_mutex_;
  bool enabled_{false};
  bool has_target_{false};
  double filtered_right_{0.0};
  double filtered_forward_{0.0};
  rclcpp::Time last_target_time_{0, 0, RCL_ROS_TIME};
  rclcpp::Time last_control_time_{0, 0, RCL_ROS_TIME};
  double previous_linear_{0.0};
  double previous_angular_{0.0};
  PidController distance_pid_;
  PidController heading_pid_;

  rclcpp::Subscription<geometry_msgs::msg::PointStamped>::SharedPtr target_subscription_;
  rclcpp::Publisher<geometry_msgs::msg::Twist>::SharedPtr cmd_vel_publisher_;
  rclcpp::Service<std_srvs::srv::SetBool>::SharedPtr enable_service_;
  rclcpp::TimerBase::SharedPtr control_timer_;
};

}  // namespace target_follower

int main(int argc, char ** argv)
{
  rclcpp::init(argc, argv);
  try {
    rclcpp::spin(std::make_shared<target_follower::TargetFollowerNode>());
  } catch (const std::exception & error) {
    RCLCPP_FATAL(rclcpp::get_logger("target_follower_node"), "%s", error.what());
  }
  rclcpp::shutdown();
  return 0;
}
