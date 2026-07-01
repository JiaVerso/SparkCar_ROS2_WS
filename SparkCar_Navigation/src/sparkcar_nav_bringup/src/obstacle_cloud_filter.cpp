#include <cmath>
#include <limits>
#include <string>
#include <vector>

#include "rclcpp/rclcpp.hpp"
#include "sensor_msgs/msg/point_cloud2.hpp"
#include "sensor_msgs/point_cloud2_iterator.hpp"

class ObstacleCloudFilter : public rclcpp::Node
{
public:
  ObstacleCloudFilter()
  : Node("obstacle_cloud_filter")
  {
    input_topic_ = declare_parameter<std::string>("input_topic", "/fastlio2/body_cloud");
    output_topic_ = declare_parameter<std::string>("output_topic", "/nav2/obstacle_cloud");

    min_x_ = declare_parameter<double>("min_x", 0.30);
    max_x_ = declare_parameter<double>("max_x", 2.0);
    min_abs_y_ = declare_parameter<double>("min_abs_y", 0.0);
    max_abs_y_ = declare_parameter<double>("max_abs_y", 0.50);
    min_z_ = declare_parameter<double>("min_z", 0.35);
    max_z_ = declare_parameter<double>("max_z", 1.0);
    min_range_ = declare_parameter<double>("min_range", 0.30);
    max_range_ = declare_parameter<double>("max_range", 2.0);

    pub_ = create_publisher<sensor_msgs::msg::PointCloud2>(output_topic_, 10);
    sub_ = create_subscription<sensor_msgs::msg::PointCloud2>(
      input_topic_,
      rclcpp::SensorDataQoS(),
      std::bind(&ObstacleCloudFilter::cloudCallback, this, std::placeholders::_1));

    RCLCPP_INFO(
      get_logger(),
      "obstacle_cloud_filter: %s -> %s",
      input_topic_.c_str(),
      output_topic_.c_str());
  }

private:
  void cloudCallback(const sensor_msgs::msg::PointCloud2::SharedPtr msg)
  {
    sensor_msgs::msg::PointCloud2 out;
    out.header = msg->header;
    out.height = 1;
    out.is_bigendian = msg->is_bigendian;
    out.is_dense = false;

    sensor_msgs::PointCloud2Modifier modifier(out);
    modifier.setPointCloud2FieldsByString(1, "xyz");
    modifier.resize(0);

    sensor_msgs::PointCloud2ConstIterator<float> in_x(*msg, "x");
    sensor_msgs::PointCloud2ConstIterator<float> in_y(*msg, "y");
    sensor_msgs::PointCloud2ConstIterator<float> in_z(*msg, "z");

    std::vector<float> kept;
    kept.reserve(msg->width * msg->height * 3);

    for (; in_x != in_x.end(); ++in_x, ++in_y, ++in_z) {
      const float x = *in_x;
      const float y = *in_y;
      const float z = *in_z;

      if (!std::isfinite(x) || !std::isfinite(y) || !std::isfinite(z)) {
        continue;
      }

      const double range = std::hypot(x, y);
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

    modifier.resize(kept.size() / 3);
    sensor_msgs::PointCloud2Iterator<float> out_x(out, "x");
    sensor_msgs::PointCloud2Iterator<float> out_y(out, "y");
    sensor_msgs::PointCloud2Iterator<float> out_z(out, "z");

    for (size_t i = 0; i < kept.size(); i += 3, ++out_x, ++out_y, ++out_z) {
      *out_x = kept[i];
      *out_y = kept[i + 1];
      *out_z = kept[i + 2];
    }

    pub_->publish(out);
  }

  std::string input_topic_;
  std::string output_topic_;

  double min_x_;
  double max_x_;
  double min_abs_y_;
  double max_abs_y_;
  double min_z_;
  double max_z_;
  double min_range_;
  double max_range_;

  rclcpp::Subscription<sensor_msgs::msg::PointCloud2>::SharedPtr sub_;
  rclcpp::Publisher<sensor_msgs::msg::PointCloud2>::SharedPtr pub_;
};

int main(int argc, char ** argv)
{
  rclcpp::init(argc, argv);
  rclcpp::spin(std::make_shared<ObstacleCloudFilter>());
  rclcpp::shutdown();
  return 0;
}
