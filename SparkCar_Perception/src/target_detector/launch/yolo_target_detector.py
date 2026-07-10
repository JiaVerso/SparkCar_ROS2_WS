# @file yolo_target_detector.py
# @author JiaVerso
# @brief 订阅 RealSense D435 发布的图像话题（Topic），然后结合 cv_bridge 将 ROS 图像转换给 YOLO 处理
# @version 0.1
# @date 2026-07-09
#
# @copyright JiaVerso (c) 2022

#!/usr/bin/env python3
import rclpy
from rclpy.node import Node
from rclpy.qos import qos_profile_sensor_data
from sensor_msgs.msg import Image, CameraInfo
from geometry_msgs.msg import PointStamped
from cv_bridge import CvBridge, CvBridgeError
import cv2
import numpy as np
import threading
import time
from ultralytics import YOLO

class YoloDetectorNode(Node):
    def __init__(self):
        super().__init__('yolo_detector_node')
        
        # --- 1. 声明参数 ---
        # 换成了最新的 yolo11n.pt，程序启动时会自动联网下载这个权重！
        self.declare_parameter('model_path', 'yolo11n.pt')
        self.declare_parameter('color_topic', '/camera/camera/color/image_raw')
        # 订阅对齐到彩色的深度图 (非常重要，保证像素一一对应)
        self.declare_parameter('depth_topic', '/camera/camera/depth/image_rect_raw')
        self.declare_parameter('camera_info_topic', '/camera/camera/color/camera_info')
        self.declare_parameter('max_depth_time_diff', 0.3)
        self.declare_parameter('display_rate', 20.0)
        self.declare_parameter('inference_imgsz', 320)
        self.declare_parameter('confidence_threshold', 0.5)
        
        model_path = self.get_parameter('model_path').get_parameter_value().string_value
        color_topic = self.get_parameter('color_topic').get_parameter_value().string_value
        depth_topic = self.get_parameter('depth_topic').get_parameter_value().string_value
        info_topic = self.get_parameter('camera_info_topic').get_parameter_value().string_value
        self.max_depth_time_diff = self.get_parameter('max_depth_time_diff').get_parameter_value().double_value
        display_rate = self.get_parameter('display_rate').get_parameter_value().double_value
        self.inference_imgsz = self.get_parameter('inference_imgsz').get_parameter_value().integer_value
        self.confidence_threshold = self.get_parameter('confidence_threshold').get_parameter_value().double_value

        self.bridge = CvBridge()
        self.get_logger().info(f"正在加载 YOLO 模型: {model_path} ...")
        self.model = YOLO(model_path)

        # --- 2. 相机内参变量 ---
        self.camera_intrinsics_ready = False
        self.fx = self.fy = self.cx_int = self.cy_int = 0.0

        # --- 3. 创建发布者 ---
        # 发布检测到的 3D 目标点，供控制节点订阅
        self.target_pub = self.create_publisher(PointStamped, '/yolo/target_point_3d', 10)

        # --- 4. 订阅相机内参 ---
        self.info_sub = self.create_subscription(CameraInfo, info_topic, self.info_callback, qos_profile_sensor_data)

        # --- 5. 订阅 Color + Depth ---
        # 回调只缓存最新帧；YOLO 推理放到后台线程，显示界面不会被推理阻塞。
        self.color_sub = self.create_subscription(
            Image, color_topic, self.color_callback, qos_profile_sensor_data
        )
        self.depth_sub = self.create_subscription(
            Image, depth_topic, self.depth_callback, qos_profile_sensor_data
        )

        self.frame_lock = threading.Lock()
        self.latest_color_image = None
        self.latest_color_header = None
        self.latest_depth_image = None
        self.latest_depth_header = None
        self.latest_detections = []
        self.last_inference_time = 0.0
        self.last_processed_color_stamp = None
        self.color_count = 0
        self.depth_count = 0
        self.process_count = 0
        self.diagnostic_timer = self.create_timer(3.0, self.diagnostic_callback)
        self.display_timer = self.create_timer(1.0 / max(display_rate, 1.0), self.display_callback)
        self.stop_event = threading.Event()
        self.inference_thread = threading.Thread(target=self.inference_loop, daemon=True)
        self.inference_thread.start()
        
        self.get_logger().info(
            f"YOLO 3D 节点初始化完成，等待图像数据: color={color_topic}, depth={depth_topic}"
        )

    def diagnostic_callback(self):
        if self.color_count == 0 or self.depth_count == 0:
            self.get_logger().warn(
                f"等待图像中: color_count={self.color_count}, depth_count={self.depth_count}。"
                "请用 ros2 topic hz 检查 color/depth 是否都在发布。",
                throttle_duration_sec=6.0,
            )
        elif self.process_count == 0:
            self.get_logger().warn(
                "已经收到 color/depth，但还没开始处理。通常是在等待 camera_info，"
                "或者 color/depth 时间戳差距超过 max_depth_time_diff。",
                throttle_duration_sec=6.0,
            )

    def info_callback(self, msg):
        #"""只执行一次，获取相机的内参矩阵 (针孔相机模型)"""
        if not self.camera_intrinsics_ready:
            self.fx = msg.k[0]
            self.cx_int = msg.k[2]
            self.fy = msg.k[4]
            self.cy_int = msg.k[5]
            self.camera_intrinsics_ready = True
            self.get_logger().info(f"成功获取相机内参: fx={self.fx:.1f}, fy={self.fy:.1f}, cx={self.cx_int:.1f}, cy={self.cy_int:.1f}")
            # 获取到内参后，可以取消订阅以节省资源
            self.destroy_subscription(self.info_sub)

    def get_robust_depth(self, depth_image, cx, cy, window_size=5):
        # """
        # 获取鲁棒的深度值：RealSense 的单像素点可能会有空洞(返回0)，
        # 所以我们取中心点附近 5x5 窗口内非零深度值的平均值。
        # """
        h, w = depth_image.shape
        x_min = max(0, cx - window_size // 2)
        x_max = min(w, cx + window_size // 2 + 1)
        y_min = max(0, cy - window_size // 2)
        y_max = min(h, cy + window_size // 2 + 1)
        
        roi = depth_image[y_min:y_max, x_min:x_max]
        valid_pixels = roi[roi > 0] # 过滤掉 0 值 (黑洞)
        
        if len(valid_pixels) == 0:
            return 0.0 # 附近全是盲区
        return float(np.median(valid_pixels)) # 使用中位数更能抵抗噪点

    def stamp_to_sec(self, stamp):
        return stamp.sec + stamp.nanosec * 1e-9

    def depth_callback(self, msg):
        self.depth_count += 1
        try:
            if msg.encoding == "32FC1":
                cv_depth = self.bridge.imgmsg_to_cv2(msg, "32FC1")
                cv_depth = np.nan_to_num(cv_depth, nan=0.0, posinf=0.0, neginf=0.0) * 1000.0
                cv_depth = cv_depth.astype(np.uint16)
            else:
                cv_depth = self.bridge.imgmsg_to_cv2(msg, "16UC1")
        except CvBridgeError as e:
            self.get_logger().error(f"深度图转换失败: {e}", throttle_duration_sec=2.0)
            return

        with self.frame_lock:
            self.latest_depth_image = cv_depth
            self.latest_depth_header = msg.header

    def color_callback(self, color_msg):
        self.color_count += 1
        try:
            cv_color = self.bridge.imgmsg_to_cv2(color_msg, "bgr8")
        except CvBridgeError as e:
            self.get_logger().error(f"彩色图转换失败: {e}", throttle_duration_sec=2.0)
            return

        with self.frame_lock:
            self.latest_color_image = cv_color
            self.latest_color_header = color_msg.header

    def inference_loop(self):
        while rclpy.ok() and not self.stop_event.is_set():
            with self.frame_lock:
                if (
                    self.latest_color_image is None
                    or self.latest_depth_image is None
                    or self.latest_color_header is None
                    or self.latest_depth_header is None
                ):
                    color_image = None
                else:
                    color_image = self.latest_color_image.copy()
                    depth_image = self.latest_depth_image.copy()
                    color_header = self.latest_color_header
                    depth_header = self.latest_depth_header

            if color_image is None:
                time.sleep(0.02)
                continue
            if not self.camera_intrinsics_ready:
                time.sleep(0.02)
                continue

            color_stamp = (color_header.stamp.sec, color_header.stamp.nanosec)
            if color_stamp == self.last_processed_color_stamp:
                time.sleep(0.01)
                continue

            time_diff = abs(
                self.stamp_to_sec(color_header.stamp) - self.stamp_to_sec(depth_header.stamp)
            )
            if time_diff > self.max_depth_time_diff:
                self.get_logger().warn(
                    f"跳过一帧: color/depth 时间差 {time_diff:.3f}s > {self.max_depth_time_diff:.3f}s",
                    throttle_duration_sec=2.0,
                )
                time.sleep(0.02)
                continue

            self.last_processed_color_stamp = color_stamp
            self.run_yolo(color_image, depth_image, color_header)

    def run_yolo(self, cv_color, cv_depth, color_header):
        self.process_count += 1
        if self.process_count == 1:
            self.get_logger().info("已收到彩色图和深度图，开始 YOLO 推理。")

        # 运行 YOLO 推理
        results = self.model(
            cv_color,
            conf=self.confidence_threshold,
            imgsz=self.inference_imgsz,
            verbose=False,
        )

        best_target = None
        largest_area = 0
        detections = []

        # 解析检测结果，寻找最大的目标 (通常代表离车最近)
        for r in results:
            for box in r.boxes:
                cls_id = int(box.cls[0].item())
                # 可选：如果只想跟随人，可以加上 if cls_id == 0: (0是COCO数据集的人)
                
                x1, y1, x2, y2 = box.xyxy[0].cpu().numpy()
                area = (x2 - x1) * (y2 - y1)
                class_name = self.model.names[cls_id]
                detections.append(
                    {
                        "box": (int(x1), int(y1), int(x2), int(y2)),
                        "label": class_name,
                    }
                )
                
                if area > largest_area:
                    largest_area = area
                    best_target = (x1, y1, x2, y2, cls_id)

        # 如果找到了目标，进行 3D 解算
        if best_target:
            x1, y1, x2, y2, cls_id = best_target
            class_name = self.model.names[cls_id]
            
            # 1. 提取 2D 中心点
            cx = int((x1 + x2) / 2)
            cy = int((y1 + y2) / 2)

            # 2. 从深度图提取 Z 轴距离 (毫米 -> 米)
            depth_mm = self.get_robust_depth(cv_depth, cx, cy)
            Z = depth_mm / 1000.0

            if Z > 0:
                # 3. 针孔相机模型逆投影：2D 像素 (u,v) 转化为 3D 坐标 (X,Y)
                X = (cx - self.cx_int) * Z / self.fx
                Y = (cy - self.cy_int) * Z / self.fy

                # 4. 封装并发布 3D 点云消息
                target_msg = PointStamped()
                target_msg.header = color_header # 继承相机画面的时间戳和坐标系 (frame_id)
                target_msg.point.x = X # 目标相对于相机的 左右 位置
                target_msg.point.y = Y # 目标相对于相机的 上下 位置
                target_msg.point.z = Z # 目标相对于相机的 前后 距离
                
                self.target_pub.publish(target_msg)
                
                self.get_logger().info(f"追踪 {class_name} -> 3D坐标: X={X:.2f}m, Z(距离)={Z:.2f}m")

                detections.append(
                    {
                        "center": (cx, cy),
                        "label": f"{class_name} Z: {Z:.2f}m",
                    }
                )
        elif self.process_count % 30 == 0:
            self.get_logger().info("已收到图像，但当前画面没有检测到目标。")

        with self.frame_lock:
            self.latest_detections = detections
            self.last_inference_time = time.time()

    def display_callback(self):
        with self.frame_lock:
            if self.latest_color_image is None:
                return
            frame = self.latest_color_image.copy()
            detections = list(self.latest_detections)
            inference_age = time.time() - self.last_inference_time if self.last_inference_time > 0 else None

        for det in detections:
            if "box" in det:
                x1, y1, x2, y2 = det["box"]
                cv2.rectangle(frame, (x1, y1), (x2, y2), (0, 255, 0), 2)
                cv2.putText(
                    frame,
                    det["label"],
                    (x1, max(20, y1 - 8)),
                    cv2.FONT_HERSHEY_SIMPLEX,
                    0.6,
                    (0, 255, 255),
                    2,
                )
            if "center" in det:
                cx, cy = det["center"]
                cv2.circle(frame, (cx, cy), 5, (0, 0, 255), -1)
                cv2.putText(
                    frame,
                    det["label"],
                    (cx + 10, max(20, cy - 10)),
                    cv2.FONT_HERSHEY_SIMPLEX,
                    0.6,
                    (0, 255, 255),
                    2,
                )

        if inference_age is not None:
            cv2.putText(
                frame,
                f"infer age: {inference_age:.2f}s",
                (10, 25),
                cv2.FONT_HERSHEY_SIMPLEX,
                0.7,
                (255, 255, 0),
                2,
            )

        cv2.imshow("ROS 2 YOLO 3D Tracker", frame)
        cv2.waitKey(1)

    def destroy_node(self):
        self.stop_event.set()
        if self.inference_thread.is_alive():
            self.inference_thread.join(timeout=1.0)
        super().destroy_node()

def main(args=None):
    rclpy.init(args=args)
    node = YoloDetectorNode()
    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    finally:
        cv2.destroyAllWindows()
        node.destroy_node()
        rclpy.shutdown()

if __name__ == '__main__':
    main()
