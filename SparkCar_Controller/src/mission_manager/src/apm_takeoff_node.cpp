/**
 * @file apm_takeoff_node.cpp
 * @brief ArduPilot Copter 4.6 autonomous takeoff using ROS 2 and MAVROS
 * @brief wait for mavros connection；wait for local position；wait for services；GUIDED mode request timeout；arming failure retry；
 * -----  takeoff failure retry；confirm aircraft enters GUIDED mode；confirm aircraft is disarmed；check takeoff height；detect exit from GUIDED mode during flight；publish hover position after takeoff.
 *
 */

#include <chrono>
#include <memory>
#include <string>

#include "rclcpp/rclcpp.hpp"
#include "geometry_msgs/msg/pose_stamped.hpp"
#include "mavros_msgs/msg/state.hpp"
#include "mavros_msgs/srv/command_bool.hpp"
#include "mavros_msgs/srv/command_tol.hpp"
#include "mavros_msgs/srv/set_mode.hpp"

namespace {
    // 转序字符
    constexpr const char * kColorReset   = "\033[0m";
    constexpr const char * kColorRed     = "\033[31m";
    constexpr const char * kColorGreen   = "\033[32m";
    constexpr const char * kColorYellow  = "\033[33m";
    constexpr const char * kColorBlue    = "\033[34m";
    constexpr const char * kColorMagenta = "\033[35m";     // 紫红色
    constexpr const char * kColorCyan    = "\033[36m";     // 青色
    constexpr const char * kColorBold    = "\033[1m";      // 高亮粗体
    constexpr const char * kColorBoldGreen = "\033[1;32m"; // 高亮粗体绿
    constexpr const char * kBgBrightRed    = "\033[101m";  // 亮红底
    constexpr const char * kStyleBlink     = "\033[5m";    // 慢速闪烁
    
} // namespace

using namespace std::chrono_literals;

int main(int argc, char ** argv)
{
  rclcpp::init(argc, argv);

  auto node = rclcpp::Node::make_shared("apm_takeoff_node");

  // 声明参数
  const double takeoff_height_ =
  node->declare_parameter<double>("takeoff_height", 2.0);

  // 高度容忍值
  const double arrival_tolerance_ =
  node->declare_parameter<double>("arrival_tolerance", 0.3);

  // 循环频率
  const double control_rate_ =
  node->declare_parameter<double>("control_rate", 20.0);

  // 切换模式
  const std::string guided_mode_ =
  node->declare_parameter<std::string>("guided_mode", "GUIDED");

  // MAVROS话题
  const std::string state_topic_ =
  node->declare_parameter<std::string>("state_topic", "/mavros/state");

  const std::string local_pose_topic_ =
  node->declare_parameter<std::string>("local_pose_topic", "/mavros/local_position/pose");

  const std::string setpoint_topic_ =
  node->declare_parameter<std::string>("setpoint_topic", "/mavros/setpoint_position/local"); 

  // MAVROS服务
  const std::string set_mode_service_ =
  node->declare_parameter<std::string>("set_mode_service", "/mavros/set_mode");

  const std::string arming_service_ =
  node->declare_parameter<std::string>("arming_service", "/mavros/cmd/arming");

  const std::string takeoff_service_ =
  node->declare_parameter<std::string>("takeoff_service", "/mavros/cmd/takeoff");


  // 创建发布、订阅、Client 客户端
  mavros_msgs::msg::State current_state;
  geometry_msgs::msg::PoseStamped current_pose;

  bool state_received = false;
  bool pose_received = false;

  // mavros status
  auto state_sub =
    node->create_subscription<mavros_msgs::msg::State>(
    state_topic_,
    rclcpp::SensorDataQoS(),
    [&](const mavros_msgs::msg::State::SharedPtr msg) {
      current_state = *msg;
      state_received = true;
    });

  // mavros local position
  auto pose_sub =
    node->create_subscription<geometry_msgs::msg::PoseStamped>(
    local_pose_topic_,
    rclcpp::SensorDataQoS(),
    [&](const geometry_msgs::msg::PoseStamped::SharedPtr msg) {
      current_pose = *msg;
      pose_received = true;
    });

  // mavros 本地位置目标发布器
  auto local_position_pub =
    node->create_publisher<geometry_msgs::msg::PoseStamped>(
    setpoint_topic_,
    10);

  // Client for checking mode
  auto mode_client =
    node->create_client<mavros_msgs::srv::SetMode>(
    set_mode_service_);

  // Arm and Disarm Client
  auto arming_client =
    node->create_client<mavros_msgs::srv::CommandBool>(
    arming_service_);

  // Take-off and Land Client
  auto takeoff_client =
    node->create_client<mavros_msgs::srv::CommandTOL>(
    takeoff_service_);

  rclcpp::Rate rate(control_rate_);

  // --------------------------------------------------------------------------
  // 等待 MAVROS 连接
  // --------------------------------------------------------------------------
  RCLCPP_INFO(
    node->get_logger(),
    "%s Waiting for FCU connection and local position... %s",
    kColorCyan, kColorReset);

  while (rclcpp::ok() &&
    (!state_received || !current_state.connected || !pose_received))
  {
    rclcpp::spin_some(node);
    rate.sleep();
  }

  if (!rclcpp::ok()) {
    rclcpp::shutdown();
    return 0;
  }

  // 保存起飞位置
  const double start_x = current_pose.pose.position.x;
  const double start_y = current_pose.pose.position.y;
  const double start_z = current_pose.pose.position.z;
  const double target_z = start_z + takeoff_height_;

  // 保存起飞时姿态
  const auto start_orientation = current_pose.pose.orientation;

  RCLCPP_INFO(
    node->get_logger(),
    "%s FCU connected. Start position: x=%.2f, y=%.2f, z=%.2f %s",
    kColorCyan, start_x, start_y, start_z, kColorReset);

  // --------------------------------------------------------------------------
  //  等待 MAVROS 服务
  // --------------------------------------------------------------------------
  RCLCPP_INFO(node->get_logger(), "Waiting for MAVROS services...");

  while (rclcpp::ok() &&
    (!mode_client->wait_for_service(1s) ||
    !arming_client->wait_for_service(1s) ||
    !takeoff_client->wait_for_service(1s)))
  {
    RCLCPP_WARN(
      node->get_logger(),
      "%s MAVROS services are not available %s",
      kColorMagenta, kColorReset);

    rclcpp::spin_some(node);
  }

  // --------------------------------------------------------------------------
  // 切换模式
  // --------------------------------------------------------------------------
  while (rclcpp::ok() && current_state.mode != guided_mode_) {
    auto request =
      std::make_shared<mavros_msgs::srv::SetMode::Request>();

    request->base_mode = 0;
    request->custom_mode = guided_mode_;

    auto future = mode_client->async_send_request(request);

    // get the result of the service 
    const auto result = rclcpp::spin_until_future_complete(
      node, future, 3s);

    if (result == rclcpp::FutureReturnCode::SUCCESS) {
      if (future.get()->mode_sent) {
        RCLCPP_INFO(
          node->get_logger(),
          "%s %s mode request accepted %s",
          kColorBold, guided_mode_.c_str(), kColorReset);
      } else {
        RCLCPP_WARN(
          node->get_logger(),
          "%s ArduPilot rejected %s mode request %s",
          kColorMagenta, guided_mode_.c_str(), kColorReset);
      }
    } else {
      RCLCPP_WARN(
        node->get_logger(),
        "%s %s mode service call timed out %s",
        kColorMagenta, guided_mode_.c_str(), kColorReset);
    }

    // 等待状态消息确认模式切换
    for (int i = 0; rclcpp::ok() && i < 20; ++i) {
      rclcpp::spin_some(node);
      rate.sleep();
    }
  }

  RCLCPP_INFO(node->get_logger(), "Vehicle is in %s mode", guided_mode_.c_str());

  // --------------------------------------------------------------------------
  //   解锁
  // --------------------------------------------------------------------------
  while (rclcpp::ok() && !current_state.armed) {
    auto request =
      std::make_shared<mavros_msgs::srv::CommandBool::Request>();

    request->value = true;

    auto future = arming_client->async_send_request(request);

    const auto result = rclcpp::spin_until_future_complete(
      node, future, 3s);

    if (result == rclcpp::FutureReturnCode::SUCCESS) {
      if (future.get()->success) {
        RCLCPP_INFO(
          node->get_logger(),
          "%s Arming request accepted %s",
          kColorBold, kColorReset);
      } else {
        RCLCPP_WARN(
          node->get_logger(),
          "%s Arming rejected, MAV_RESULT=%u %s",
          kColorMagenta, future.get()->result, kColorReset);
      }
    } else {
      RCLCPP_WARN(
        node->get_logger(),
        "%s Arming service call timed out %s",
        kColorMagenta, kColorReset);
    }

    // 等待
    for (int i = 0; rclcpp::ok() && i < 20; ++i) {
      rclcpp::spin_some(node);
      rate.sleep();
    }
  }

  RCLCPP_INFO(node->get_logger(), "Vehicle armed");

  // --------------------------------------------------------------------------
  //  起飞
  // --------------------------------------------------------------------------
  bool takeoff_accepted = false;

  while (rclcpp::ok() && !takeoff_accepted) {
    auto request =
      std::make_shared<mavros_msgs::srv::CommandTOL::Request>();

    request->min_pitch = 0.0;
    request->yaw = 0.0;
    request->latitude = 0.0;
    request->longitude = 0.0;
    request->altitude = takeoff_height_;

    auto future = takeoff_client->async_send_request(request);

    const auto result = rclcpp::spin_until_future_complete(
      node, future, 3s);

    if (result == rclcpp::FutureReturnCode::SUCCESS) {
      if (future.get()->success) {
        takeoff_accepted = true;

        RCLCPP_INFO(
          node->get_logger(),
          "Takeoff command accepted, height: %.2f m",
          takeoff_height_);
      } else {
        RCLCPP_WARN(
          node->get_logger(),
          "%s Takeoff rejected, MAV_RESULT=%u %s",
          kColorCyan, future.get()->result, kColorReset);
      }
    } else {
      RCLCPP_WARN(
        node->get_logger(),
        "%s Takeoff service call timed out %s",
        kColorCyan, kColorReset);
    }

    rclcpp::spin_some(node);
    rate.sleep();
  }

  // --------------------------------------------------------------------------
  //   到达目标高度
  // --------------------------------------------------------------------------
  while (rclcpp::ok() &&
    current_pose.pose.position.z < target_z - arrival_tolerance_)
  {
    RCLCPP_INFO_THROTTLE(
      node->get_logger(),
      *node->get_clock(),
      1000,
      "%s Taking off: current z=%.2f, target z=%.2f %s",
      kColorCyan,
      current_pose.pose.position.z,
      target_z,
      kColorReset);

    if (current_state.mode != guided_mode_) {
      RCLCPP_ERROR(
        node->get_logger(),
        "%s Vehicle Missing %s mode during takeoff %s",
        kColorRed, guided_mode_.c_str(), kColorReset);
      break;
    }

    if (!current_state.armed) {
      RCLCPP_ERROR(
        node->get_logger(),
        "%s Vehicle disarmed during takeoff %s",
        kColorRed, kColorReset);
      break;
    }

    rclcpp::spin_some(node);
    rate.sleep();
  }

  // --------------------------------------------------------------------------
  //   发布位置目标
  // --------------------------------------------------------------------------
  geometry_msgs::msg::PoseStamped target_pose;

  target_pose.header.frame_id = "map";
  target_pose.pose.position.x = start_x;
  target_pose.pose.position.y = start_y;
  target_pose.pose.position.z = target_z;
  target_pose.pose.orientation = start_orientation;

  RCLCPP_INFO(
    node->get_logger(),
    "%s Takeoff completed. Holding position at %.2f m %s",
    kColorGreen,
    target_z,
    kColorReset);

  while (rclcpp::ok()) {
    rclcpp::spin_some(node);

    if (current_state.mode != guided_mode_) {
      RCLCPP_WARN_THROTTLE(
        node->get_logger(),
        *node->get_clock(),
        1000,
        "%s Vehicle is no longer in %s mode %s",
        kColorRed, guided_mode_.c_str(), kColorReset);
    }

    if (!current_state.armed) {
      RCLCPP_WARN(
        node->get_logger(),
        "Vehicle disarmed, stopping setpoint publication");
      break;
    }

    target_pose.header.stamp = node->now();
    local_position_pub->publish(target_pose);

    rate.sleep();
  }

  rclcpp::shutdown();
  return 0;
}