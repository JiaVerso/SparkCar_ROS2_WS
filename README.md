# SparkCar_ROS2_WS

SparkCar ROS 2 workspace for the SparkCar robot platform. This repository groups the Hunter SE chassis driver, Livox MID360 perception/localization stack, Nav2 bringup, Hybrid A* planner demo, and map conversion tools.

The README style follows the lightweight structure commonly used by AgileX/Hunter ROS repositories: packages, communication setup, build, launch, and safety notes.

## Contents

- [Packages](#packages)
- [Environment](#environment)
- [CAN Interface Setup](#can-interface-setup)
- [Build](#build)
- [Basic Usage](#basic-usage)
- [Start Hunter SE Chassis](#start-hunter-se-chassis)
- [Start Livox MID360](#start-livox-mid360)
- [PGO Mapping And Save PCD](#pgo-mapping-and-save-pcd)
- [Convert PCD To PGM For Nav2](#convert-pcd-to-pgm-for-nav2)
- [Start FAST-LIO2 Localizer And Nav2](#start-fast-lio2-localizer-and-nav2)
- [Nav2 Velocity Output](#nav2-velocity-output)
- [Twist To Ackermann Converter](#twist-to-ackermann-converter)
- [Hybrid A* Demo](#hybrid-a-demo)
- [Useful Topics And Checks](#useful-topics-and-checks)
- [Safety Notes](#safety-notes)
- [License](#license)

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
cd /home/jiaverso/Desktop/SparkCar_ROS2_WS/HunterSE_Driver
source /opt/ros/humble/setup.bash
colcon build --symlink-install
```

Build navigation packages:

```bash
cd /home/jiaverso/Desktop/SparkCar_ROS2_WS/SparkCar_Navigation
source /opt/ros/humble/setup.bash
colcon build --symlink-install
```

If perception/tools source packages are present on the target machine, build them the same way:

```bash
cd /home/jiaverso/Desktop/SparkCar_ROS2_WS/SparkCar_Perception
source /opt/ros/humble/setup.bash
colcon build --symlink-install

cd /home/jiaverso/Desktop/SparkCar_ROS2_WS/SparkCar_Tools
source /opt/ros/humble/setup.bash
colcon build --symlink-install
```

Source installed workspaces after build:

```bash
source /opt/ros/humble/setup.bash
source /home/jiaverso/Desktop/SparkCar_ROS2_WS/HunterSE_Driver/install/setup.bash
source /home/jiaverso/Desktop/SparkCar_ROS2_WS/SparkCar_Perception/install/setup.bash
source /home/jiaverso/Desktop/SparkCar_ROS2_WS/SparkCar_Tools/install/setup.bash
source /home/jiaverso/Desktop/SparkCar_ROS2_WS/SparkCar_Navigation/install/setup.bash
```

## Basic Usage

### Start Hunter SE Chassis

```bash
source /opt/ros/humble/setup.bash
source /home/jiaverso/Desktop/SparkCar_ROS2_WS/HunterSE_Driver/install/setup.bash

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

Start the Livox MID360 driver before mapping or localization:

```bash
source /opt/ros/humble/setup.bash
source /home/jiaverso/Desktop/SparkCar_ROS2_WS/SparkCar_Perception/install/setup.bash

ros2 launch livox_ros_driver2 msg_MID360_launch.py
```

Check lidar and IMU topics:

```bash
ros2 topic hz /livox/lidar
ros2 topic hz /livox/imu
```

Expected raw sensor topics:

- `/livox/lidar`
- `/livox/imu`

### PGO Mapping And Save PCD

Run the mapping helper:

```bash
cd /home/jiaverso/Desktop/SparkCar_ROS2_WS
bash scripts/ControlCommand.sh
```

Manual mapping sequence:

```bash
source /opt/ros/humble/setup.bash
source /home/jiaverso/Desktop/SparkCar_ROS2_WS/SparkCar_Perception/install/setup.bash
source /home/jiaverso/Desktop/SparkCar_ROS2_WS/SparkCar_Tools/install/setup.bash

# Terminal 1: start lidar
ros2 launch livox_ros_driver2 msg_MID360_launch.py

# Terminal 2: start PGO mapping
ros2 launch pgo pgo_launch.py
```

`pgo_launch.py` starts both FAST-LIO2 and the PGO node:

```text
livox_ros_driver2 -> /livox/lidar, /livox/imu
FAST-LIO2         -> /fastlio2/lio_odom, /fastlio2/body_cloud
PGO              -> optimized map and /pgo/save_maps service
```

Optional rosbag recording during mapping:

```bash
mkdir -p ~/Desktop/bags

ros2 bag record \
  /livox/lidar \
  /livox/imu \
  /fastlio2/lio_odom \
  /fastlio2/body_cloud \
  -o ~/Desktop/bags/mapping_001
```

Save PGO map:

```bash
mkdir -p ~/Desktop/Save_Map
ros2 service call /pgo/save_maps interface/srv/SaveMaps \
  "{file_path: '$HOME/Desktop/Save_Map', save_patches: true}"
```

The PGO service saves the merged map as:

```text
~/Desktop/Save_Map/map.pcd
```

If the navigation workflow expects `main.pcd`, copy or rename it:

```bash
cp ~/Desktop/Save_Map/map.pcd ~/Desktop/Save_Map/main.pcd
```

If `save_patches: true` is used, PGO also saves patch files under:

```text
~/Desktop/Save_Map/patches/
```

### Convert PCD To PGM For Nav2

`pcd2pgm` converts a `.pcd` point cloud map into a 2D occupancy grid and publishes it on `/map`. Then `map_saver_cli` saves the Nav2 `.pgm` and `.yaml` map files.

The default `pcd2pgm` config currently reads:

```text
~/Desktop/Save_Map/map.pcd
```

If you copied the map to `main.pcd`, update:

```text
SparkCar_Tools/src/pcd2pgm/config/pcd2pgm.yaml
```

Set:

```yaml
pcd2pgm:
  ros__parameters:
    pcd_file: /home/jiaverso/Desktop/Save_Map/main.pcd
```

Rebuild or use the installed config that matches your runtime environment. Then run:

```bash
source /opt/ros/humble/setup.bash
source /home/jiaverso/Desktop/SparkCar_ROS2_WS/SparkCar_Tools/install/setup.bash

ros2 launch pcd2pgm pcd2pgm_launch.py \
  params_file:=/home/jiaverso/Desktop/SparkCar_ROS2_WS/SparkCar_Tools/src/pcd2pgm/config/pcd2pgm.yaml
```

In another terminal, save the published `/map` as `main.pgm` and `main.yaml`:

```bash
source /opt/ros/humble/setup.bash
mkdir -p /home/jiaverso/Desktop/SparkCar_ROS2_WS/SparkCar_Navigation/src/sparkcar_nav_bringup/maps

ros2 run nav2_map_server map_saver_cli \
  -f /home/jiaverso/Desktop/SparkCar_ROS2_WS/SparkCar_Navigation/src/sparkcar_nav_bringup/maps/main
```

This creates:

```text
SparkCar_Navigation/src/sparkcar_nav_bringup/maps/main.pgm
SparkCar_Navigation/src/sparkcar_nav_bringup/maps/main.yaml
```

Confirm that `main.yaml` points to `main.pgm`:

```yaml
image: main.pgm
resolution: 0.05
origin: [...]
```

### Start FAST-LIO2 Localizer And Nav2

Run the navigation helper:

```bash
cd /home/jiaverso/Desktop/SparkCar_ROS2_WS
bash scripts/nav2_test.sh
```

Manual navigation sequence:

```bash
source /opt/ros/humble/setup.bash
source /home/jiaverso/Desktop/SparkCar_ROS2_WS/SparkCar_Perception/install/setup.bash
source /home/jiaverso/Desktop/SparkCar_ROS2_WS/SparkCar_Tools/install/setup.bash
source /home/jiaverso/Desktop/SparkCar_ROS2_WS/SparkCar_Navigation/install/setup.bash

# Terminal 1: start Livox MID360
ros2 launch livox_ros_driver2 msg_MID360_launch.py

# Terminal 2: start FAST-LIO2 localizer
ros2 launch localizer localizer_launch.py

# Terminal 3: start Nav2
ros2 launch sparkcar_nav_bringup nav_bringup.launch.py
```

`localizer_launch.py` starts:

```text
fastlio2/lio_node
localizer/localizer_node
localizer RViz
```

`nav_bringup.launch.py` starts:

```text
map_server
planner_server
controller_server
smoother_server
behavior_server
bt_navigator
lifecycle_manager_navigation
rviz2
```

If needed, start Micro XRCE-DDS Agent:

```bash
sudo MicroXRCEAgent serial --dev /dev/ttyUSB1 -b 115200 -v 6
```

Disable RViz:

```bash
ros2 launch sparkcar_nav_bringup nav_bringup.launch.py use_rviz:=false
```

### Nav2 Velocity Output

Nav2 computes velocity commands from the planned path and publishes:

```text
/cmd_vel
```

Check output:

```bash
ros2 topic echo /cmd_vel
ros2 topic hz /cmd_vel
```

If using the Hunter SE ROS2 driver directly, `hunter_base` subscribes to `/cmd_vel`, so the control chain is:

```text
Nav2 controller_server -> /cmd_vel -> hunter_base -> CAN -> Hunter SE
```

In this case, do not run `twist_to_ackermann` unless a separate lower-level controller is explicitly subscribing to `/ackermann_cmd`.

### Twist To Ackermann Converter

The converter subscribes to `/cmd_vel` and publishes `/ackermann_cmd`.

```bash
ros2 run sparkcar_nav_bringup twist_to_ackermann
```

Use this only when the lower-level controller expects `ackermann_msgs/msg/AckermannDrive` on `/ackermann_cmd`.

Parameters:

- `wheelbase`: default `0.60`
- `max_steering_angle`: default `0.52`
- `min_speed_for_steering`: default `0.05`
- `cmd_vel_topic`: default `/cmd_vel`
- `ackermann_topic`: default `/ackermann_cmd`

### Hybrid A* Demo

```bash
source /opt/ros/humble/setup.bash
source /home/jiaverso/Desktop/SparkCar_ROS2_WS/SparkCar_Navigation/install/setup.bash

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
