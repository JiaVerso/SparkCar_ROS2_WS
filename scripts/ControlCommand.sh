#!/usr/bin/env bash

# Simple SparkCar mapping launcher.
# Run this in Ubuntu/ROS2 Humble, not in Windows PowerShell.

set -e

WS=/home/jiaverso/Desktop/SparkCar_ROS2_WS
MAP_DIR=/home/jiaverso/Desktop/Save_Map
BAG_DIR=/home/jiaverso/Desktop/bags/mapping_$(date +%Y%m%d_%H%M%S)

LIVOX_LAUNCH=$WS/SparkCar_Perception/install/livox_ros_driver2/share/livox_ros_driver2/launch_ROS2/msg_MID360_launch.py

source /opt/ros/humble/setup.bash
source $WS/SparkCar_Perception/install/setup.bash
source $WS/SparkCar_Tools/install/setup.bash

mkdir -p $MAP_DIR
mkdir -p /home/jiaverso/Desktop/bags

echo "1. Start Livox radar..."
ros2 launch $LIVOX_LAUNCH &
RADAR_PID=$!

sleep 5

echo "2. Start rosbag recording..."
ros2 bag record /livox/lidar /livox/imu -o $BAG_DIR &
BAG_PID=$!

sleep 2

echo "3. Start PGO mapping..."
ros2 launch pgo pgo_launch.py &
MAPPING_PID=$!

echo
echo "Mapping is running now."
echo "Drive or push the car to finish mapping."
echo "When the route is finished, come back here and press Enter to save the map."
read

echo "4. Save map..."
ros2 service call /pgo/save_maps interface/srv/SaveMaps "{file_path: '$MAP_DIR', save_patches: true}"

echo "5. Stop rosbag recording..."
kill -INT $BAG_PID
sleep 3

echo
echo "Map saved to: $MAP_DIR"
echo "Bag saved to: $BAG_DIR"
echo
echo "Press Ctrl+C to stop radar and mapping."
wait $MAPPING_PID

