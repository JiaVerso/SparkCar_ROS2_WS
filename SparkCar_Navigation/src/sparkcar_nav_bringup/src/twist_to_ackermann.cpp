#include <algorithm>   
#include <cmath>       
#include <memory>      
#include <string>      

#include "ackermann_msgs/msg/ackermann_drive_stamped.hpp"  
#include "geometry_msgs/msg/twist.hpp"                  
#include "rclcpp/rclcpp.hpp"                              

class TwistToAckermann : public rclcpp::Node
{
public:
  TwistToAckermann()
  : Node("twist_to_ackermann")  // 创建 ROS2 节点，节点名叫 twist_to_ackermann
  {
    wheelbase_ = declare_parameter<double>("wheelbase", 0.60);  // 声明参数 车轴距，单位 m
    max_steering_angle_ = declare_parameter<double>("max_steering_angle", 0.52);  
    min_speed_for_steering_ = declare_parameter<double>("min_speed_for_steering", 0.05);  

    cmd_vel_topic_ = declare_parameter<std::string>("cmd_vel_topic", "/cmd_vel");  // 输入 topic
    ackermann_topic_ = declare_parameter<std::string>("ackermann_topic", "/ackermann_cmd");  // 输出 topic

    pub_ = create_publisher<ackermann_msgs::msg::AckermannDriveStamped>(
      ackermann_topic_, 10);  // 创建发布器，发布阿克曼消息

    sub_ = create_subscription<geometry_msgs::msg::Twist>(
      cmd_vel_topic_,
      10,
      std::bind(&TwistToAckermann::cmdVelCallback, this, std::placeholders::_1));  

    RCLCPP_INFO(get_logger(), "twist_to_ackermann started");  // 打印启动日志
  }

private:
  void cmdVelCallback(const geometry_msgs::msg::Twist::ConstSharedPtr msg)
  {
    const double v = msg->linear.x;      
    const double omega = msg->angular.z; 

    double steering_angle = 0.0;         // 前轮转角

    if (std::abs(v) > min_speed_for_steering_) {
      steering_angle = std::atan(wheelbase_ * omega / v);  // 前轮转角 = atan(轴距 * 角速度 / 线速度)
    } else {
      steering_angle = 0.0;  
    }

    steering_angle = std::clamp(
      steering_angle,
      -max_steering_angle_,
      max_steering_angle_);  // 限制转角

    ackermann_msgs::msg::AckermannDriveStamped out;  /
    out.header.stamp = now();                        // 时间戳
    out.header.frame_id = "base_link";               
    out.drive.speed = v;                             
    out.drive.steering_angle = steering_angle;      

    pub_->publish(out);                              // 发布 /ackermann_cmd
  }

  double wheelbase_;                 // 轴距
  double max_steering_angle_;        // 最大前轮转角
  double min_speed_for_steering_;    // 最小计算速度

  std::string cmd_vel_topic_;        // 输入 topic 名
  std::string ackermann_topic_;      

  rclcpp::Subscription<geometry_msgs::msg::Twist>::SharedPtr sub_;  // 订阅器
  rclcpp::Publisher<ackermann_msgs::msg::AckermannDriveStamped>::SharedPtr pub_;  // 发布器
};

int main(int argc, char ** argv)
{
  rclcpp::init(argc, argv);                         // 初始化 ROS2
  rclcpp::spin(std::make_shared<TwistToAckermann>()); // 运行节点，持续接收 /cmd_vel
  rclcpp::shutdown();                               // 退出 ROS2
  return 0;                                    
}