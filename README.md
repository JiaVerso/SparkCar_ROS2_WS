# SparkCar_ROS2_WS

SparkCar ROS 2 workspace for the SparkCar robot platform. This repository groups the Hunter SE chassis driver, Livox MID360 perception/localization stack, Nav2 bringup, Hybrid A* planner demo, and map conversion tools.

The README style follows the lightweight structure commonly used by AgileX/Hunter ROS repositories: packages, communication setup, build, launch, and safety notes.

## Packages

```text
SparkCar_ROS2_WS
+-- HunterSE_Driver
|   +-- src
|       +-- hunter_base        # Hunter SE / Hunter 2 chassis ROS 2 node
|       +-- hunter_msgs        # Hunter status and command messages
|       +-- ugv_sdk            # Weston Robot / AgileX UGV SDK
+-- SparkCar_Perception
|   +-- src                    # Livox MID360 and FAST-LIO2 related source tree
|   +-- install                # Installed perception packages: livox_ros_driver2, localizer, pgo, interface, etc.
+-- SparkCar_Navigation
|   +-- src
|       +-- sparkcar_nav_bringup
|       |   +-- launch         # Nav2 launch files
|       |   +-- config         # Nav2 parameters and waypoints
|       |   +-- maps           # Default map
|       |   +-- src            # twist_to_ackermann converter
|       +-- hybrid_astar_planner
|           +-- launch         # Hybrid A* demo launch
|           +-- config         # Planner parameters
|           +-- rviz           # RViz config
+-- SparkCar_Tools
|   +-- install/pcd2pgm        # PCD to occupancy grid map conversion tool
+-- scripts
    +-- ControlCommand.sh      # Mapping helper script
    +-- nav2_test.sh           # Navigation stack helper script
    +-- process.md             # Mapping workflow notes
```

## Environment

Recommended runtime environment:

- Ubuntu 22.04
- ROS 2 Humble
- SocketCAN enabled CAN adapter for Hunter SE chassis
- Livox MID360 lidar
- Micro XRCE-DDS Agent if the lower controller uses serial XRCE-DDS

Install common ROS dependencies:

```bash
sudo apt update
sudo apt install -y \
  python3-colcon-common-extensions \
  ros-humble-rclcpp \
  ros-humble-geometry-msgs \
  ros-humble-nav-msgs \
  ros-humble-sensor-msgs \
  ros-humble-ackermann-msgs \
  ros-humble-tf2 \
  ros-humble-tf2-ros \
  ros-humble-tf2-geometry-msgs \
  ros-humble-navigation2 \
  ros-humble-nav2-bringup \
  ros-humble-nav2-map-server \
  ros-humble-rviz2 \
  libasio-dev \
  libeigen3-dev \
  libgoogle-glog-dev
```

## CAN Interface Setup

Hunter chassis communication uses SocketCAN. The default launch parameter is `port_name:=can0`.

Bring up CAN at 500 kbit/s:

```bash
sudo ip link set can0 down
sudo ip link set can0 type can bitrate 500000
sudo ip link set can0 up
```

Check CAN traffic:

```bash
candump can0
```

## Build

This repository is organized as several ROS 2 workspaces. Build each workspace from its own directory.

Build Hunter SE driver:

```bash
cd ~/SparkCar_ROS2_WS/HunterSE_Driver
source /opt/ros/humble/setup.bash
colcon build --symlink-install
```

Build navigation packages:

```bash
cd ~/SparkCar_ROS2_WS/SparkCar_Navigation
source /opt/ros/humble/setup.bash
colcon build --symlink-install
```

If perception/tools source packages are present on the target machine, build them the same way:

```bash
cd ~/SparkCar_ROS2_WS/SparkCar_Perception
source /opt/ros/humble/setup.bash
colcon build --symlink-install

cd ~/SparkCar_ROS2_WS/SparkCar_Tools
source /opt/ros/humble/setup.bash
colcon build --symlink-install
```

Source installed workspaces after build:

```bash
source /opt/ros/humble/setup.bash
source ~/SparkCar_ROS2_WS/HunterSE_Driver/install/setup.bash
source ~/SparkCar_ROS2_WS/SparkCar_Perception/install/setup.bash
source ~/SparkCar_ROS2_WS/SparkCar_Tools/install/setup.bash
source ~/SparkCar_ROS2_WS/SparkCar_Navigation/install/setup.bash
```

## Basic Usage

### Start Hunter SE Chassis

```bash
source /opt/ros/humble/setup.bash
source ~/SparkCar_ROS2_WS/HunterSE_Driver/install/setup.bash

ros2 launch hunter_base hunter_base.launch.py port_name:=can0 robot_model:=hunter_se
```

Common launch arguments:

- `port_name`: CAN interface, default `can0`
- `robot_model`: `hunter_se` or `hunter2`, default `hunter2`
- `odom_frame`: odometry frame, default `odom`
- `base_frame`: base frame, default `base_link`
- `odom_topic_name`: odometry topic, default `odom`

The chassis node subscribes to `/cmd_vel`, publishes odometry on the configured odometry topic, publishes TF from `odom_frame` to `base_frame`, and publishes chassis status on `/hunter_status`.

Send a simple test command:

```bash
ros2 topic pub /cmd_vel geometry_msgs/msg/Twist \
  "{linear: {x: 0.2}, angular: {z: 0.0}}" -r 10
```

Stop the robot:

```bash
ros2 topic pub /cmd_vel geometry_msgs/msg/Twist \
  "{linear: {x: 0.0}, angular: {z: 0.0}}" -1
```

### Start Livox MID360

The helper scripts expect the installed Livox launch file:

```bash
source /opt/ros/humble/setup.bash
source ~/SparkCar_ROS2_WS/SparkCar_Perception/install/setup.bash

ros2 launch livox_ros_driver2 msg_MID360_launch.py
```

Check lidar and IMU topics:

```bash
ros2 topic hz /livox/lidar
ros2 topic hz /livox/imu
```

### Mapping With PGO

Run the mapping helper:

```bash
cd ~/SparkCar_ROS2_WS
bash scripts/ControlCommand.sh
```

Manual mapping sequence:

```bash
source /opt/ros/humble/setup.bash
source ~/SparkCar_ROS2_WS/SparkCar_Perception/install/setup.bash
source ~/SparkCar_ROS2_WS/SparkCar_Tools/install/setup.bash

ros2 launch livox_ros_driver2 msg_MID360_launch.py
ros2 launch pgo pgo_launch.py
```

Save PGO map:

```bash
mkdir -p ~/Desktop/Save_Map
ros2 service call /pgo/save_maps interface/srv/SaveMaps \
  "{file_path: '$HOME/Desktop/Save_Map', save_patches: true}"
```

Convert PCD map to occupancy grid map:

```bash
ros2 launch pcd2pgm pcd2pgm_launch.py
ros2 run nav2_map_server map_saver_cli -f ~/Desktop/Save_Map/nav2_map
```

The installed `pcd2pgm` default parameters use:

- `pcd_file`: `/home/jiaverso/Desktop/Save_Map/map.pcd`
- `map_resolution`: `0.05`
- `map_topic_name`: `map`

Update `pcd2pgm.yaml` or pass a custom `params_file` if your map path is different.

### Start Localization And Nav2

Run the navigation helper:

```bash
cd ~/SparkCar_ROS2_WS
bash scripts/nav2_test.sh
```

Manual navigation sequence:

```bash
source /opt/ros/humble/setup.bash
source ~/SparkCar_ROS2_WS/SparkCar_Perception/install/setup.bash
source ~/SparkCar_ROS2_WS/SparkCar_Tools/install/setup.bash
source ~/SparkCar_ROS2_WS/SparkCar_Navigation/install/setup.bash

ros2 launch livox_ros_driver2 msg_MID360_launch.py
ros2 launch localizer localizer_launch.py
ros2 launch sparkcar_nav_bringup nav_bringup.launch.py
ros2 run sparkcar_nav_bringup twist_to_ackermann
```

If needed, start Micro XRCE-DDS Agent:

```bash
sudo MicroXRCEAgent serial --dev /dev/ttyUSB1 -b 115200 -v 6
```

`sparkcar_nav_bringup` starts map server, planner server, controller server, smoother server, behavior server, BT navigator, lifecycle manager, and RViz by default.

Disable RViz:

```bash
ros2 launch sparkcar_nav_bringup nav_bringup.launch.py use_rviz:=false
```

### Twist To Ackermann Converter

The converter subscribes to `/cmd_vel` and publishes `/ackermann_cmd`.

```bash
ros2 run sparkcar_nav_bringup twist_to_ackermann
```

Parameters:

- `wheelbase`: default `0.60`
- `max_steering_angle`: default `0.52`
- `min_speed_for_steering`: default `0.05`
- `cmd_vel_topic`: default `/cmd_vel`
- `ackermann_topic`: default `/ackermann_cmd`

### Hybrid A* Demo

```bash
source /opt/ros/humble/setup.bash
source ~/SparkCar_ROS2_WS/SparkCar_Navigation/install/setup.bash

ros2 launch hybrid_astar_planner hybrid_astar_test.py
```

## Useful Topics And Checks

```bash
ros2 topic list
ros2 topic echo /hunter_status
ros2 topic echo /odom
ros2 topic echo /cmd_vel
ros2 topic echo /ackermann_cmd
ros2 topic echo /plan
ros2 lifecycle get /map_server
ros2 lifecycle get /planner_server
ros2 lifecycle get /controller_server
ros2 lifecycle get /bt_navigator
ros2 run tf2_ros tf2_echo map body
```

## Safety Notes

- Always lift the robot wheels or keep the robot in an open area for the first motion test.
- Verify the CAN interface and emergency stop before sending `/cmd_vel`.
- Start with low linear speed, for example `0.1` to `0.2 m/s`.
- Keep a terminal ready to publish zero velocity or stop the launch process.
- Confirm TF frames and map alignment before enabling autonomous navigation.

## License

This repository contains code from multiple components. See [LICENSE](LICENSE) and package-level license files for details.
