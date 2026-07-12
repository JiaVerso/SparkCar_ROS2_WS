/*
# @file yolo_target_detector.cpp
# @author JiaVerso
# @brief  A C++ ROS 2 node designed to capture video frames from a USB camera
          Uses the PyTorch C++ API to run an exported YOLO11 TorchScript model
          Publishes the pixel coordinates of the centers of detected objects
          Features a multi-threaded image processing architecture
# @version 0.1
# @date 2026-07-10
#
# @copyright JiaVerso (c) 2026
*/

#include <algorithm>
#include <array>
#include <atomic>     // 原子操作，线程安全标记
#include <chrono>     // 时间库
#include <cmath>
#include <cstdint>
#include <memory>
#include <mutex>      
#include <stdexcept>
#include <string>
#include <thread> 
#include <vector>

#include <geometry_msgs/msg/point_stamped.hpp>
#include <librealsense2/rs.hpp>
#include <librealsense2/rsutil.h>
#include <opencv2/core.hpp>
#include <opencv2/highgui.hpp>
#include <opencv2/imgproc.hpp>
#include <opencv2/videoio.hpp>
#include <rclcpp/rclcpp.hpp>
#include <ATen/Parallel.h>
#include <torch/script.h>

using namespace std::chrono_literals;

namespace target_detector
{

struct Detection
{
  bool found{false};
  cv::Rect box;       // 矩形框坐标
  float confidence{0.0F};     // 置信度
  int class_id{-1};
};

class YoloUsbDetector : public rclcpp::Node
{
public:
  YoloUsbDetector()
  : Node("yolo_usb_detector")
  {
    // 声明外部可配置参数
    model_path_ = declare_parameter<std::string>("model_path", "");

    // D435 彩色图相关配置
    camera_backend_ = declare_parameter<std::string>("camera_backend", "realsense");
    camera_device_ = declare_parameter<std::string>("camera_device", "/dev/video4");
    realsense_serial_ = declare_parameter<std::string>("realsense_serial", "");
    camera_index_ = declare_parameter<int>("camera_index", 0);
    camera_width_ = declare_parameter<int>("camera_width", 640);
    camera_height_ = declare_parameter<int>("camera_height", 480);
    camera_fps_ = declare_parameter<int>("camera_fps", 30);

    // D435 深度图相关配置
    depth_window_size_ = declare_parameter<int>("depth_window_size", 7);
    min_depth_ = declare_parameter<double>("min_depth", 0.2);
    max_depth_ = declare_parameter<double>("max_depth", 8.0);

    use_mjpeg_ = declare_parameter<bool>("use_mjpeg", true);
    input_size_ = declare_parameter<int>("input_size", 320);
    confidence_threshold_ = declare_parameter<double>("confidence_threshold", 0.7);

    // 非极大值抑制
    nms_threshold_ = declare_parameter<double>("nms_threshold", 0.45);
    target_class_id_ = declare_parameter<int>("target_class_id", 0);
    target_class_name_ = declare_parameter<std::string>("target_class_name", "target");

    // 推理频率
    inference_frequency_ = declare_parameter<double>("inference_frequency", 10.0);

    // 分配线程核心
    inference_threads_ = declare_parameter<int>("inference_threads", 4);
    show_image_ = declare_parameter<bool>("show_image", true);
    frame_id_ = declare_parameter<std::string>("frame_id", "usb_camera_pixel");

    // 输出话题
    output_topic_ = declare_parameter<std::string>("output_topic", "/yolo/target_pixel");
    camera_optical_frame_id_ = declare_parameter<std::string>(
      "camera_optical_frame_id", "camera_color_optical_frame");
    point_3d_topic_ = declare_parameter<std::string>(
      "point_3d_topic", "/yolo/target_point_3d");

    if (model_path_.empty()) {
      throw std::runtime_error("parameter 'model_path' must point to a YOLO11 TorchScript model");
    }
    if (input_size_ <= 0 || inference_frequency_ <= 0.0) {
      throw std::runtime_error("input_size and inference_frequency must be positive");
    }
    if (depth_window_size_ <= 0 || min_depth_ < 0.0 || max_depth_ <= min_depth_) {
      throw std::runtime_error("invalid depth window or depth range parameters");
    }
    if (camera_backend_ != "realsense" && camera_backend_ != "v4l2") {
      throw std::runtime_error("camera_backend must be 'realsense' or 'v4l2'");
    }

    load_model();
    open_camera();

    target_publisher_ =
      create_publisher<geometry_msgs::msg::PointStamped>(output_topic_, rclcpp::QoS(10));
    target_3d_publisher_ =
      create_publisher<geometry_msgs::msg::PointStamped>(point_3d_topic_, rclcpp::QoS(10));

    // 原子变量标记
    running_.store(true);
    capture_thread_ = std::thread(&YoloUsbDetector::capture_loop, this);

    const auto period = std::chrono::duration<double>(1.0 / inference_frequency_);
    inference_timer_ = create_wall_timer(
      std::chrono::duration_cast<std::chrono::nanoseconds>(period),
      std::bind(&YoloUsbDetector::inference_callback, this));

    RCLCPP_INFO(
      get_logger(),
      "YOLO detector ready: backend=%s, model=%s, target_class_id=%d, topic=%s",
      camera_backend_.c_str(), model_path_.c_str(), target_class_id_, output_topic_.c_str());
  }

  // 析构函数
  ~YoloUsbDetector() override
  {
    running_.store(false);
    if (realsense_pipeline_) {
      try {
        realsense_pipeline_->stop();
      } catch (const rs2::error &) {
      }
    }
    if (capture_thread_.joinable()) {
      capture_thread_.join();
    }
    camera_.release();
    if (show_image_) {
      cv::destroyAllWindows();
    }
  }

private:
  void load_model()
  {
    RCLCPP_INFO(get_logger(), "Loading YOLO11 TorchScript model: %s", model_path_.c_str());
    try {
      at::set_num_threads(std::max(1, inference_threads_));
      model_ = torch::jit::load(model_path_, torch::kCPU);
      model_.eval();

      torch::NoGradGuard no_grad;
      const torch::Tensor warmup_input = torch::zeros(
        {1, 3, input_size_, input_size_}, torch::TensorOptions().dtype(torch::kFloat32));
      const torch::Tensor warmup_output = model_.forward({warmup_input}).toTensor();
      if (warmup_output.dim() != 3 || warmup_output.size(0) != 1) {
        throw std::runtime_error("TorchScript model is not a YOLO detection model");
      }
      RCLCPP_INFO(
        get_logger(), "Model warmup succeeded, output shape: [%ld, %ld, %ld]",
        warmup_output.size(0), warmup_output.size(1), warmup_output.size(2));
    } catch (const c10::Error & error) {
      throw std::runtime_error(std::string("failed to load TorchScript model: ") + error.what());
    }
  }

  void open_camera()
  {
    if (camera_backend_ == "realsense") {
      open_realsense_camera();
      return;
    }

    const bool opened = camera_device_.empty() ?
      camera_.open(camera_index_, cv::CAP_V4L2) :
      camera_.open(camera_device_, cv::CAP_V4L2);
    if (!opened) {
      throw std::runtime_error(
              "failed to open USB camera " +
              (camera_device_.empty() ? std::to_string(camera_index_) : camera_device_));
    }

    if (use_mjpeg_) {
      camera_.set(cv::CAP_PROP_FOURCC, cv::VideoWriter::fourcc('M', 'J', 'P', 'G'));
    }
    camera_.set(cv::CAP_PROP_FRAME_WIDTH, camera_width_);
    camera_.set(cv::CAP_PROP_FRAME_HEIGHT, camera_height_);
    camera_.set(cv::CAP_PROP_FPS, camera_fps_);
    camera_.set(cv::CAP_PROP_BUFFERSIZE, 1);

    RCLCPP_INFO(
      get_logger(), "Camera opened: %.0fx%.0f @ %.1f FPS",
      camera_.get(cv::CAP_PROP_FRAME_WIDTH), camera_.get(cv::CAP_PROP_FRAME_HEIGHT),
      camera_.get(cv::CAP_PROP_FPS));
  }

  void open_realsense_camera()
  {
    try {
      realsense_pipeline_ = std::make_unique<rs2::pipeline>();
      rs2::config config;
      if (!realsense_serial_.empty()) {
        config.enable_device(realsense_serial_);
      }
      config.enable_stream(
        RS2_STREAM_COLOR, camera_width_, camera_height_, RS2_FORMAT_BGR8, camera_fps_);
      config.enable_stream(
        RS2_STREAM_DEPTH, camera_width_, camera_height_, RS2_FORMAT_Z16, camera_fps_);

      const rs2::pipeline_profile profile = realsense_pipeline_->start(config);
      const rs2::device device = profile.get_device();
      const rs2::video_stream_profile color_profile =
        profile.get_stream(RS2_STREAM_COLOR).as<rs2::video_stream_profile>();
      color_intrinsics_ = color_profile.get_intrinsics();
      depth_scale_ = device.first<rs2::depth_sensor>().get_depth_scale();
      realsense_align_ = std::make_unique<rs2::align>(RS2_STREAM_COLOR);
      RCLCPP_INFO(
        get_logger(),
        "RealSense opened: %s, serial=%s, RGB-D=%dx%d @ %d FPS, depth_scale=%.6f",
        device.get_info(RS2_CAMERA_INFO_NAME),
        device.get_info(RS2_CAMERA_INFO_SERIAL_NUMBER),
        camera_width_, camera_height_, camera_fps_, depth_scale_);
    } catch (const rs2::error & error) {
      throw std::runtime_error(
              std::string("failed to start RealSense color stream: ") + error.what() +
              ". Close realsense-viewer before starting this node.");
    }
  }

  void capture_loop()
  {
    while (running_.load() && rclcpp::ok()) {
      cv::Mat frame;
      cv::Mat depth_frame;
      if (camera_backend_ == "realsense") {
        try {
          // 等待图像数据
          const rs2::frameset frames = realsense_pipeline_->wait_for_frames(1000);
          
          // 深度图对齐二维像素
          const rs2::frameset aligned_frames = realsense_align_->process(frames);

          // 获取一帧图像数据
          const rs2::video_frame color = aligned_frames.get_color_frame();
          const rs2::depth_frame depth = aligned_frames.get_depth_frame();

          // 判断是否为空
          if (!color || !depth) {
            continue;
          }

          // 浅拷贝机制
          const cv::Mat color_view(
            cv::Size(color.get_width(), color.get_height()), CV_8UC3,
            const_cast<void *>(color.get_data()), cv::Mat::AUTO_STEP);

          const cv::Mat depth_view(
            cv::Size(depth.get_width(), depth.get_height()), CV_16UC1,
            const_cast<void *>(depth.get_data()), cv::Mat::AUTO_STEP);
            
          frame = color_view.clone();
          depth_frame = depth_view.clone();
        } catch (const rs2::error & error) {
          if (!running_.load()) {
            break;
          }
          RCLCPP_WARN_THROTTLE(
            get_logger(), *get_clock(), 2000,
            "Failed to read a RealSense color frame: %s", error.what());
          continue;
        }
      } else {
        if (!camera_.read(frame) || frame.empty()) {
          RCLCPP_WARN_THROTTLE(
            get_logger(), *get_clock(), 2000, "Failed to read a frame from the USB camera");
          std::this_thread::sleep_for(20ms);
          continue;
        }
      }

      // 自动加锁
      std::lock_guard<std::mutex> lock(frame_mutex_);
      latest_frame_ = frame;
      latest_depth_frame_ = depth_frame;
      ++frame_sequence_;
    }
  }

  // 等比例缩放，填充边界
  cv::Mat letterbox(const cv::Mat & image, float & scale, int & pad_x, int & pad_y) const
  {
    scale = std::min(
      static_cast<float>(input_size_) / static_cast<float>(image.cols),
      static_cast<float>(input_size_) / static_cast<float>(image.rows));

    const int resized_width = static_cast<int>(std::round(image.cols * scale));
    const int resized_height = static_cast<int>(std::round(image.rows * scale));
    pad_x = (input_size_ - resized_width) / 2;
    pad_y = (input_size_ - resized_height) / 2;

    cv::Mat resized;
    cv::resize(image, resized, cv::Size(resized_width, resized_height));
    cv::Mat padded(input_size_, input_size_, CV_8UC3, cv::Scalar(114, 114, 114));
    resized.copyTo(padded(cv::Rect(pad_x, pad_y, resized_width, resized_height)));
    return padded;
  }

  Detection detect(const cv::Mat & frame)
  {

    float scale = 1.0F;
    int pad_x = 0;
    int pad_y = 0;

    // 图像预处理
    const cv::Mat input = letterbox(frame, scale, pad_x, pad_y);
    cv::Mat rgb;
    cv::cvtColor(input, rgb, cv::COLOR_BGR2RGB);

    // 归一化
    cv::Mat normalized;
    rgb.convertTo(normalized, CV_32FC3, 1.0 / 255.0);

    torch::Tensor input_tensor = torch::from_blob(
      normalized.data, {1, input_size_, input_size_, 3}, torch::kFloat32);
    input_tensor = input_tensor.permute({0, 3, 1, 2}).contiguous();

    // Forward Inference
    torch::NoGradGuard no_grad;
    torch::Tensor predictions = model_.forward({input_tensor}).toTensor().to(torch::kCPU).contiguous();
    if (predictions.dim() != 3 || predictions.size(0) != 1) {
      RCLCPP_ERROR_THROTTLE(
        get_logger(), *get_clock(), 5000,
        "Unexpected YOLO output shape. Export a detection model with fixed input size");
      return {};
    }

    const int64_t axis_1 = predictions.size(1);
    const int64_t axis_2 = predictions.size(2);
    const bool feature_first = axis_1 < axis_2;
    const int64_t feature_count = feature_first ? axis_1 : axis_2;
    const int64_t candidate_count = feature_first ? axis_2 : axis_1;
    const int class_count = feature_count - 4;
    if (class_count <= 0 || target_class_id_ < 0 || target_class_id_ >= class_count) {
      RCLCPP_ERROR_THROTTLE(
        get_logger(), *get_clock(), 5000,
        "Invalid model output (%ld features) or target_class_id=%d", feature_count,
        target_class_id_);
      return {};
    }

    const float * output = predictions.data_ptr<float>();
    const auto value_at = [output, feature_first, feature_count, candidate_count](
      int64_t candidate, int64_t feature) -> float
      {
        return feature_first ?
               output[feature * candidate_count + candidate] :
               output[candidate * feature_count + feature];
      };

    std::vector<cv::Rect> boxes;
    std::vector<float> confidences;
    boxes.reserve(candidate_count);
    confidences.reserve(candidate_count);

    // 置信度阈值过滤
    for (int64_t candidate = 0; candidate < candidate_count; ++candidate) {
      const float confidence = value_at(candidate, 4 + target_class_id_);
      if (confidence < static_cast<float>(confidence_threshold_)) {
        continue;
      }

      const float center_x = value_at(candidate, 0);
      const float center_y = value_at(candidate, 1);
      const float width = value_at(candidate, 2);
      const float height = value_at(candidate, 3);

      // 提取坐标
      int left = static_cast<int>((center_x - width * 0.5F - pad_x) / scale);
      int top = static_cast<int>((center_y - height * 0.5F - pad_y) / scale);
      int right = static_cast<int>((center_x + width * 0.5F - pad_x) / scale);
      int bottom = static_cast<int>((center_y + height * 0.5F - pad_y) / scale);

      left = std::clamp(left, 0, frame.cols - 1);
      top = std::clamp(top, 0, frame.rows - 1);
      right = std::clamp(right, 0, frame.cols - 1);
      bottom = std::clamp(bottom, 0, frame.rows - 1);
      if (right <= left || bottom <= top) {
        continue;
      }

      boxes.emplace_back(left, top, right - left, bottom - top);
      confidences.push_back(confidence);
    }

    std::vector<int> sorted_indices(boxes.size());
    for (size_t index = 0; index < sorted_indices.size(); ++index) {
      sorted_indices[index] = static_cast<int>(index);
    }

    //按照置信度排序
    std::sort(
      sorted_indices.begin(), sorted_indices.end(),
      [&confidences](int left, int right) {return confidences[left] > confidences[right];});

    std::vector<int> kept_indices;
    for (const int index : sorted_indices) {
      bool suppressed = false;
      for (const int kept : kept_indices) {
        const cv::Rect intersection = boxes[index] & boxes[kept];
        const int union_area = boxes[index].area() + boxes[kept].area() - intersection.area();
        const float iou = union_area > 0 ?
          static_cast<float>(intersection.area()) / static_cast<float>(union_area) : 0.0F;
        if (iou > static_cast<float>(nms_threshold_)) {
          suppressed = true;
          break;
        }
      }
      if (!suppressed) {
        kept_indices.push_back(index);
      }
    }

    // 同一类别多目标，根据像素面积选择
    Detection best;
    int best_area = -1;
    for (const int index : kept_indices) {
      const int area = boxes[index].area();
      if (area > best_area) {
        best_area = area;
        best.found = true;
        best.box = boxes[index];
        best.confidence = confidences[index];
        best.class_id = target_class_id_;
      }
    }
    return best;
  }

  bool deproject_target(
    const cv::Mat & depth_image, int pixel_u, int pixel_v,
    std::array<float, 3> & point_3d) const
  {
    if (depth_image.empty() || depth_image.type() != CV_16UC1 || depth_scale_ <= 0.0F) {
      return false;
    }

    const int half_window = depth_window_size_ / 2;
    const int x_min = std::max(0, pixel_u - half_window);
    const int x_max = std::min(depth_image.cols - 1, pixel_u + half_window);
    const int y_min = std::max(0, pixel_v - half_window);
    const int y_max = std::min(depth_image.rows - 1, pixel_v + half_window);

    std::vector<uint16_t> valid_depths;
    valid_depths.reserve((x_max - x_min + 1) * (y_max - y_min + 1));
    for (int y = y_min; y <= y_max; ++y) {
      const uint16_t * row = depth_image.ptr<uint16_t>(y);
      for (int x = x_min; x <= x_max; ++x) {
        const uint16_t raw_depth = row[x];
        const float depth_m = static_cast<float>(raw_depth) * depth_scale_;
        if (raw_depth > 0 && depth_m >= min_depth_ && depth_m <= max_depth_) {
          valid_depths.push_back(raw_depth);
        }
      }
    }
    if (valid_depths.empty()) {
      return false;
    }

    const auto median = valid_depths.begin() + valid_depths.size() / 2;
    std::nth_element(valid_depths.begin(), median, valid_depths.end());
    const float depth_m = static_cast<float>(*median) * depth_scale_;
    const float pixel[2] = {
      static_cast<float>(pixel_u), static_cast<float>(pixel_v)};
    rs2_deproject_pixel_to_point(point_3d.data(), &color_intrinsics_, pixel, depth_m);
    return std::isfinite(point_3d[0]) && std::isfinite(point_3d[1]) &&
           std::isfinite(point_3d[2]);
  }

  // 推理回调
  void inference_callback()
  {
    cv::Mat frame;
    cv::Mat depth_frame;
    uint64_t sequence = 0;

    // 局部作用域
    {
      std::lock_guard<std::mutex> lock(frame_mutex_);
      if (latest_frame_.empty() || frame_sequence_ == processed_sequence_) {
        return;
      }
      frame = latest_frame_.clone();
      depth_frame = latest_depth_frame_.clone();
      sequence = frame_sequence_;
    }
    processed_sequence_ = sequence;

    Detection detection;
    try {
      detection = detect(frame);
    } catch (const std::exception & error) {
      RCLCPP_ERROR_THROTTLE(
        get_logger(), *get_clock(), 3000, "YOLO inference failed: %s", error.what());
      return;
    }

    if (detection.found) {

      // 矩形中心框坐标
      const int center_u = detection.box.x + detection.box.width / 2;
      const int center_v = detection.box.y + detection.box.height / 2;

      geometry_msgs::msg::PointStamped target_pixel;
      target_pixel.header.stamp = now();
      target_pixel.header.frame_id = frame_id_;
      target_pixel.point.x = static_cast<double>(center_u);
      target_pixel.point.y = static_cast<double>(center_v);
      target_pixel.point.z = static_cast<double>(detection.confidence);
      target_publisher_->publish(target_pixel);

      std::array<float, 3> point_3d{};
      const bool has_3d_point = camera_backend_ == "realsense" &&
        deproject_target(depth_frame, center_u, center_v, point_3d);

      // 获取矩形框深度值
      if (has_3d_point) {
        geometry_msgs::msg::PointStamped target_3d;
        target_3d.header.stamp = target_pixel.header.stamp;
        target_3d.header.frame_id = camera_optical_frame_id_;
        target_3d.point.x = point_3d[0];
        target_3d.point.y = point_3d[1];
        target_3d.point.z = point_3d[2];
        target_3d_publisher_->publish(target_3d);
      }

      if (show_image_) {
        cv::rectangle(frame, detection.box, cv::Scalar(0, 255, 0), 2);
        cv::circle(frame, cv::Point(center_u, center_v), 5, cv::Scalar(0, 0, 255), -1);
        std::string label = target_class_name_ + " " +
          cv::format("%.2f", detection.confidence);

        if (has_3d_point) {
          label += cv::format(" Z=%.2fm", point_3d[2]);
        }
        cv::putText(
          frame, label, cv::Point(detection.box.x, std::max(20, detection.box.y - 8)),
          cv::FONT_HERSHEY_SIMPLEX, 0.6, cv::Scalar(0, 255, 255), 2);
      }
    }

    if (show_image_) {
      cv::line(
        frame, cv::Point(frame.cols / 2 - 10, frame.rows / 2),
        cv::Point(frame.cols / 2 + 10, frame.rows / 2), cv::Scalar(255, 255, 0), 1);
      cv::line(
        frame, cv::Point(frame.cols / 2, frame.rows / 2 - 10),
        cv::Point(frame.cols / 2, frame.rows / 2 + 10), cv::Scalar(255, 255, 0), 1);
      cv::imshow("YOLO11 USB Target Detector", frame);
      cv::waitKey(1);
    }
  }

  std::string model_path_;
  std::string camera_backend_;
  std::string camera_device_;
  std::string realsense_serial_;
  std::string target_class_name_;
  std::string frame_id_;
  std::string output_topic_;
  std::string camera_optical_frame_id_;
  std::string point_3d_topic_;
  int camera_index_{0};
  int camera_width_{640};
  int camera_height_{480};
  int camera_fps_{30};
  int depth_window_size_{7};
  int input_size_{320};
  int target_class_id_{0};
  int inference_threads_{4};
  bool use_mjpeg_{true};
  bool show_image_{true};
  double confidence_threshold_{0.5};
  double nms_threshold_{0.45};
  double inference_frequency_{10.0};
  double min_depth_{0.2};
  double max_depth_{8.0};

  cv::VideoCapture camera_;
  std::unique_ptr<rs2::pipeline> realsense_pipeline_;
  std::unique_ptr<rs2::align> realsense_align_;
  rs2_intrinsics color_intrinsics_{};
  float depth_scale_{0.0F};
  torch::jit::script::Module model_;
  std::atomic<bool> running_{false};
  std::thread capture_thread_;
  std::mutex frame_mutex_;
  cv::Mat latest_frame_;
  cv::Mat latest_depth_frame_;
  uint64_t frame_sequence_{0};
  uint64_t processed_sequence_{0};

  rclcpp::Publisher<geometry_msgs::msg::PointStamped>::SharedPtr target_publisher_;
  rclcpp::Publisher<geometry_msgs::msg::PointStamped>::SharedPtr target_3d_publisher_;
  
  // 定时器
  rclcpp::TimerBase::SharedPtr inference_timer_;
};

}  // namespace target_detector

int main(int argc, char ** argv)
{
  // 初始化DDS 
  rclcpp::init(argc, argv);
  try {
    // 事件轮询
    rclcpp::spin(std::make_shared<target_detector::YoloUsbDetector>());
  } catch (const std::exception & error) {
    // 获取异常
    RCLCPP_FATAL(rclcpp::get_logger("yolo_usb_detector"), "%s", error.what());
  }
  rclcpp::shutdown();
  return 0;
}
