# target_follower

ROS 2 C++ target-following controller for the Hunter SE Ackermann platform.

The node subscribes to `/yolo/target_point_3d`, where the D435 optical frame is:

- `point.x`: target position to the camera right, meters
- `point.y`: target position downward, unused by the ground vehicle controller
- `point.z`: target position forward, meters

It publishes `geometry_msgs/msg/Twist` on `/cmd_vel`. Distance controls
`linear.x`; the target bearing `atan2(-point.x, point.z)` controls `angular.z`.
The angular command is constrained by the Hunter SE minimum turning radius.

Both distance and heading loops use the reusable C++ `PidController` interface.
The current follower is deliberately configured as P-only: `Ki=Kd=Kf=0`, with
`distance_kp` and `heading_kp` supplied by the YAML file. Safety checks,
acceleration limiting and Ackermann constraints are applied after P control.

## Build

```bash
cd /home/jiaverso/Desktop/SparkCar_ROS2_WS/SparkCar_Controller
source /opt/ros/humble/setup.bash
colcon build --packages-select target_follower --symlink-install
source install/setup.bash
```

## Run

Start the Hunter base driver and YOLO detector first. Do not run Nav2 controller
output on `/cmd_vel` at the same time.

```bash
ros2 launch target_follower target_follower.launch.py
```

Reverse is disabled for safety. Only after checking the area behind the vehicle,
it can be enabled with `allow_reverse:=true`.

The controller starts disabled. Verify the target topic, raise the wheels for
the first test, then enable following:

```bash
ros2 topic echo /yolo/target_point_3d
ros2 service call /target_follower_node/set_enabled std_srvs/srv/SetBool "{data: true}"
```

Disable and stop immediately:

```bash
ros2 service call /target_follower_node/set_enabled std_srvs/srv/SetBool "{data: false}"
```

The node sends zero velocity when disabled or when the target has not been
updated within `target_timeout`. It also stops immediately inside
`emergency_stop_distance`. Reverse following is disabled by default.
