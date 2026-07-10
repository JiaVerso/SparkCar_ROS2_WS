# target_detector

ROS 2 C++ YOLO11 detector with two camera backends:

- `realsense`: directly opens the D435 color stream with librealsense2
- `v4l2`: opens a normal USB camera with OpenCV `/dev/video*`

It does not subscribe to RealSense ROS topics and does not use depth data.

## Output

The node publishes `geometry_msgs/msg/PointStamped` on `/yolo/target_pixel`:

- `point.x`: bounding-box center pixel `u`
- `point.y`: bounding-box center pixel `v`
- `point.z`: detection confidence in `[0, 1]`
- `header.frame_id`: `usb_camera_pixel`

These values are image measurements, not metric 3D coordinates.

With `camera_backend:=realsense`, the node also aligns the D435 depth image to
the RGB image and publishes `/yolo/target_point_3d`:

- `point.x`: rightward position in the color optical frame, meters
- `point.y`: downward position in the color optical frame, meters
- `point.z`: forward distance, meters
- `header.frame_id`: `camera_color_optical_frame`

Depth is calculated from the median valid value in a configurable window around
the detection center. No 3D message is published when that window has no valid
depth measurement.

## Export YOLO11

The C++ node uses the LibTorch runtime already included with the installed
Python `torch` package. Export the Ultralytics `.pt` model to TorchScript:

```bash
cd src/target_detector/models
yolo export \
  model=../launch/yolo11n.pt \
  format=torchscript \
  imgsz=320 \
  optimize=False
```

Make sure the resulting file is named `models/yolo11n.torchscript`.

For a custom-trained model, use its `.pt` file and set `target_class_id` to the
class index from the training dataset. A one-class model normally uses ID `0`.

## Build and run

```bash
cd /home/jiaverso/Desktop/SparkCar_ROS2_WS/SparkCar_Perception
source /opt/ros/humble/setup.bash
colcon build --packages-select target_detector --symlink-install
source install/setup.bash
ros2 launch target_detector yolo_usb_detector.launch.py \
  target_class_id:=0
```

Close `realsense-viewer` before launching because it otherwise owns the D435.
The default backend starts the D435 color stream at `640x480@30`.

For a normal V4L2 USB camera:

```bash
ros2 launch target_detector yolo_usb_detector.launch.py \
  camera_backend:=v4l2 \
  camera_device:=/dev/video0 \
  target_class_id:=0
```

Check available USB video devices with:

```bash
v4l2-ctl --list-devices
```

Edit `config/yolo_usb_detector.yaml` to change camera resolution, inference
size, confidence threshold, target name, display, or output topic.

The build discovers LibTorch from the active `python3` installation. Source the
same Python environment that contains `torch` before running `colcon build`.
