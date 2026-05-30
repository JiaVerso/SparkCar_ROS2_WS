#!/usr/bin/env bash

# Simple SparkCar mapping launcher.
# Run this in Ubuntu/ROS2 Humble, not in Windows PowerShell.

# 失败自动停止
set -e

WS=/home/jiaverso/Desktop/SparkCar_ROS2_WS
MAP_DIR=/home/jiaverso/Desktop/Save_Map

    # 获取系统时间戳$(date +%Y%m%d_%H%M%S)
BAG_DIR=/home/jiaverso/Desktop/bags/mapping_$(date +%Y%m%d_%H%M%S)

LIVOX_LAUNCH=$WS/SparkCar_Perception/install/livox_ros_driver2/share/livox_ros_driver2/launch_ROS2/msg_MID360_launch.py

source /opt/ros/humble/setup.bash
source $WS/SparkCar_Perception/install/setup.bash
source $WS/SparkCar_Tools/install/setup.bash


echo "1. Start Livox radar Mid360..."
    # &（保持节点静默运行）
ros2 launch $LIVOX_LAUNCH &
    # 获取后台进程
RADAR_PID=$!

sleep 5

echo "2. Start fastlio2 localization..."

ros2 launch localizer localizer_launch.py

sleep 2

echo "3. Start nav2 launch..."

ros2 launch localizer localizer_launch.py

echo
echo "nav2 is running now."
echo "Drive or push the car to finish testing."
echo "When the route is finished, come back here and press Enter to finish the testing."
# 在此暂停，监听键盘输入
read

