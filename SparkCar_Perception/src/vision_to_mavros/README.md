# vision_to_mavros

ROS 2 bridge from VINS-Fusion `nav_msgs/msg/Odometry` to MAVROS external odometry.

## Coordinate convention

- VINS input is expected to be gravity aligned and right handed, with Z up.
- The default world transform is identity; no ENU/NED axis swap is performed.
- Output pose is local ENU-compatible in `map` and body orientation is FLU in `base_link`.
- MAVROS performs the ROS ENU/FLU to autopilot NED/FRD conversion.
- VINS cannot observe absolute East from camera and IMU alone. Use `yaw_offset_deg`
  only after measuring the horizontal alignment, or configure the flight EKF not to
  fuse external-vision yaw.
- VINS-Fusion velocity `Vs` is normally expressed in the VINS world frame. The
  bridge rotates it into body FLU because `nav_msgs/Odometry.twist` is defined in
  `child_frame_id`.

## Build and run

```bash
cd ~/Desktop/SparkCar_ROS2_WS/SparkCar_Perception
source /opt/ros/humble/setup.bash
colcon build --packages-select vision_to_mavros --symlink-install
source install/setup.bash

ros2 launch vision_to_mavros vision_to_mavros.launch.py
```

For rosbag playback, play the bag with `--clock` and launch with simulated time:

```bash
ros2 bag play /path/to/bag --clock
ros2 launch vision_to_mavros vision_to_mavros.launch.py use_sim_time:=true
```

## Validation

```bash
ros2 topic info /mavros/odometry/in -v
ros2 topic hz /mavros/odometry/in
ros2 topic echo /mavros/odometry/in --once
ros2 topic echo /vision_to_mavros/valid
```

The bridge preserves the VINS measurement timestamp. It rejects invalid
quaternions, NaN/Inf, stale or non-monotonic timestamps, long gaps and pose jumps.
With `latch_faults: true`, publishing remains stopped after a fault. Reset only
after the aircraft is safe and stationary:

```bash
ros2 service call /vision_to_mavros/reset std_srvs/srv/Trigger '{}'
```

The default covariance values are initial test values, not measured sensor
statistics. Tune them from repeatable ground tests before enabling flight control.
