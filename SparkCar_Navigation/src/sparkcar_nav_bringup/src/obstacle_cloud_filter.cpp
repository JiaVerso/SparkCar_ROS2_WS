#include <cmath>     // 提供数学函数，如计算距离的 std::hypot 和绝对值 std::abs
#include <limits>    // 提供数值极限，这里用于配合 std::isfinite 检查无效数字(NaN)
#include <string>    // 字符串类
#include <vector>    // 动态数组容器

#include "rclcpp/rclcpp.hpp"                        // ROS2 C++ 核心库
#include "sensor_msgs/msg/point_cloud2.hpp"         // ROS2 标准的点云消息接口
#include "sensor_msgs/point_cloud2_iterator.hpp"    // 极其高效的点云数据迭代器库 (避免了引入庞大的 PCL 库)

class ObstacleCloudFilter : public rclcpp::Node
{
public:
  ObstacleCloudFilter()
  : Node("obstacle_cloud_filter")
  {
    // 声明并获取 ROS2 参数
    input_topic_ = declare_parameter<std::string>("input_topic", "/fastlio2/body_cloud");
    output_topic_ = declare_parameter<std::string>("output_topic", "/nav2/obstacle_cloud");
    clearing_topic_ = declare_parameter<std::string>("clearing_topic", "/nav2/clearing_cloud");

    min_x_ = declare_parameter<double>("min_x", 0.30);
    max_x_ = declare_parameter<double>("max_x", 2.0);
    min_abs_y_ = declare_parameter<double>("min_abs_y", 0.0);
    max_abs_y_ = declare_parameter<double>("max_abs_y", 0.50);
    min_z_ = declare_parameter<double>("min_z", 0.35);
    max_z_ = declare_parameter<double>("max_z", 1.0);
    min_range_ = declare_parameter<double>("min_range", 0.30);
    max_range_ = declare_parameter<double>("max_range", 2.0);

    clear_min_x_ = declare_parameter<double>("clear_min_x", 0.30);
    clear_max_x_ = declare_parameter<double>("clear_max_x", 4.0);
    clear_min_abs_y_ = declare_parameter<double>("clear_min_abs_y", 0.0);
    clear_max_abs_y_ = declare_parameter<double>("clear_max_abs_y", 2.5);
    clear_min_z_ = declare_parameter<double>("clear_min_z", -0.50);
    clear_max_z_ = declare_parameter<double>("clear_max_z", 1.50);
    clear_min_range_ = declare_parameter<double>("clear_min_range", 0.30);
    clear_max_range_ = declare_parameter<double>("clear_max_range", 4.0);
    clear_stride_ = declare_parameter<int>("clear_stride", 2);
    if (clear_stride_ < 1) {
      clear_stride_ = 1;
    }

    pub_ = create_publisher<sensor_msgs::msg::PointCloud2>(output_topic_, 10);
    clearing_pub_ = create_publisher<sensor_msgs::msg::PointCloud2>(clearing_topic_, 10);
    sub_ = create_subscription<sensor_msgs::msg::PointCloud2>(
      input_topic_,
      rclcpp::SensorDataQoS(),
      std::bind(&ObstacleCloudFilter::cloudCallback, this, std::placeholders::_1));

    // 打印初始化信息
    RCLCPP_INFO(
      get_logger(),
      "obstacle_cloud_filter: %s -> mark %s, clear %s",
      input_topic_.c_str(),
      output_topic_.c_str(),
      clearing_topic_.c_str());
  }

private:
  // 点云处理回调函数 
  void cloudCallback(const sensor_msgs::msg::PointCloud2::SharedPtr msg)
  {
    sensor_msgs::msg::PointCloud2 obstacle_out;
    sensor_msgs::msg::PointCloud2 clearing_out;
    
    obstacle_out.header = msg->header;
    obstacle_out.height = 1;
    obstacle_out.is_bigendian = msg->is_bigendian;
    obstacle_out.is_dense = false;

    clearing_out.header = msg->header;
    clearing_out.height = 1;
    clearing_out.is_bigendian = msg->is_bigendian;
    clearing_out.is_dense = false;

    sensor_msgs::PointCloud2Modifier obstacle_modifier(obstacle_out);
    obstacle_modifier.setPointCloud2FieldsByString(1, "xyz");
    obstacle_modifier.resize(0);

    sensor_msgs::PointCloud2Modifier clearing_modifier(clearing_out);
    clearing_modifier.setPointCloud2FieldsByString(1, "xyz");
    clearing_modifier.resize(0);

    sensor_msgs::PointCloud2ConstIterator<float> in_x(*msg, "x");
    sensor_msgs::PointCloud2ConstIterator<float> in_y(*msg, "y");
    sensor_msgs::PointCloud2ConstIterator<float> in_z(*msg, "z");

    std::vector<float> kept;
    kept.reserve(msg->width * msg->height * 3);

    std::vector<float> clearing;
    clearing.reserve(msg->width * msg->height * 3 / clear_stride_);

    int valid_index = 0;
    for (; in_x != in_x.end(); ++in_x, ++in_y, ++in_z) {
      const float x = *in_x;
      const float y = *in_y;
      const float z = *in_z;

      if (!std::isfinite(x) || !std::isfinite(y) || !std::isfinite(z)) {
        continue;
      }

      const double range = std::hypot(x, y);
      if (x >= clear_min_x_ && x <= clear_max_x_ &&
          std::abs(y) >= clear_min_abs_y_ && std::abs(y) <= clear_max_abs_y_ &&
          z >= clear_min_z_ && z <= clear_max_z_ &&
          range >= clear_min_range_ && range <= clear_max_range_) {
        if ((valid_index % clear_stride_) == 0) {
          clearing.push_back(x);
          clearing.push_back(y);
          clearing.push_back(z);
        }
        ++valid_index;
      }

      if (x < min_x_ || x > max_x_) {
        continue;
      }
      if (std::abs(y) < min_abs_y_ || std::abs(y) > max_abs_y_) {
        continue;
      }
      if (z < min_z_ || z > max_z_) {
        continue;
      }
      if (range < min_range_ || range > max_range_) {
        continue;
      }

      kept.push_back(x);
      kept.push_back(y);
      kept.push_back(z);
    }

    obstacle_modifier.resize(kept.size() / 3);
    sensor_msgs::PointCloud2Iterator<float> out_x(obstacle_out, "x");
    sensor_msgs::PointCloud2Iterator<float> out_y(obstacle_out, "y");
    sensor_msgs::PointCloud2Iterator<float> out_z(obstacle_out, "z");

    for (size_t i = 0; i < kept.size(); i += 3, ++out_x, ++out_y, ++out_z) {
      *out_x = kept[i];
      *out_y = kept[i + 1];
      *out_z = kept[i + 2];
    }

    clearing_modifier.resize(clearing.size() / 3);
    sensor_msgs::PointCloud2Iterator<float> clear_x(clearing_out, "x");
    sensor_msgs::PointCloud2Iterator<float> clear_y(clearing_out, "y");
    sensor_msgs::PointCloud2Iterator<float> clear_z(clearing_out, "z");

    for (size_t i = 0; i < clearing.size(); i += 3, ++clear_x, ++clear_y, ++clear_z) {
      *clear_x = clearing[i];
      *clear_y = clearing[i + 1];
      *clear_z = clearing[i + 2];
    }

    pub_->publish(obstacle_out);
    clearing_pub_->publish(clearing_out);
  }

  std::string input_topic_;
  std::string output_topic_;
  std::string clearing_topic_;

  double min_x_;
  double max_x_;
  double min_abs_y_;
  double max_abs_y_;
  double min_z_;
  double max_z_;
  double min_range_;
  double max_range_;
  double clear_min_x_;
  double clear_max_x_;
  double clear_min_abs_y_;
  double clear_max_abs_y_;
  double clear_min_z_;
  double clear_max_z_;
  double clear_min_range_;
  double clear_max_range_;
  int clear_stride_;

  rclcpp::Subscription<sensor_msgs::msg::PointCloud2>::SharedPtr sub_;
  rclcpp::Publisher<sensor_msgs::msg::PointCloud2>::SharedPtr pub_;
  rclcpp::Publisher<sensor_msgs::msg::PointCloud2>::SharedPtr clearing_pub_;
};

int main(int argc, char ** argv)
{
  rclcpp::init(argc, argv);
  rclcpp::spin(std::make_shared<ObstacleCloudFilter>());
  rclcpp::shutdown();
  return 0;
}
