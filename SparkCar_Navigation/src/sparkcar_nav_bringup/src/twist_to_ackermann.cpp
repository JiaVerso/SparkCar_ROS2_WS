#include <algorithm>
#include <cmath>
#include <memory>
#include <string>

#include "ackermann_msgs/msg/ackermann_drive.hpp"
#include "geometry_msgs/msg/twist.hpp"
#include "rclcpp/rclcpp.hpp"

class TwistToAckermann : public rclcpp::Node
{
public:
  TwistToAckermann()
  : Node("twist_to_ackermann")
  {
    wheelbase_ = declare_parameter<double>("wheelbase", 0.60);
    max_steering_angle_ = declare_parameter<double>("max_steering_angle", 0.52);
    min_speed_for_steering_ = declare_parameter<double>("min_speed_for_steering", 0.05);

    cmd_vel_topic_ = declare_parameter<std::string>("cmd_vel_topic", "/cmd_vel");
    ackermann_topic_ = declare_parameter<std::string>("ackermann_topic", "/ackermann_cmd");

    pub_ = create_publisher<ackermann_msgs::msg::AckermannDrive>(
      ackermann_topic_, 10);

    sub_ = create_subscription<geometry_msgs::msg::Twist>(
      cmd_vel_topic_,
      10,
      std::bind(&TwistToAckermann::cmdVelCallback, this, std::placeholders::_1));

    RCLCPP_INFO(get_logger(), "twist_to_ackermann started");
  }

private:
  void cmdVelCallback(const geometry_msgs::msg::Twist::ConstSharedPtr msg)
  {
    const double v = msg->linear.x;
    const double omega = msg->angular.z;

    double steering_angle = 0.0;

    if (std::abs(v) > min_speed_for_steering_) {
      steering_angle = std::atan(wheelbase_ * omega / v);
    }

    steering_angle = std::clamp(
      steering_angle,
      -max_steering_angle_,
      max_steering_angle_);

    ackermann_msgs::msg::AckermannDrive out;

    out.steering_angle = - static_cast<float>(steering_angle);
    out.steering_angle_velocity = 0.0f;
    out.speed = - static_cast<float>(v);
    out.acceleration = 0.0f;
    out.jerk = 0.0f;

    pub_->publish(out);
  }

  double wheelbase_;
  double max_steering_angle_;
  double min_speed_for_steering_;

  std::string cmd_vel_topic_;
  std::string ackermann_topic_;

  rclcpp::Subscription<geometry_msgs::msg::Twist>::SharedPtr sub_;
  rclcpp::Publisher<ackermann_msgs::msg::AckermannDrive>::SharedPtr pub_;
};

int main(int argc, char ** argv)
{
  rclcpp::init(argc, argv);
  rclcpp::spin(std::make_shared<TwistToAckermann>());
  rclcpp::shutdown();
  return 0;
}
