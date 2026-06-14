#!/usr/bin/env bash

# Start SparkCar navigation stack in order.
# Run this script in Ubuntu/ROS2 Humble, not in Windows PowerShell.

# 任意命令失败自动退出
set -e

# 路径
WS=/home/jiaverso/Desktop/SparkCar_ROS2_WS
SERIAL_DEV=/dev/ttyUSB1
BAUDRATE=115200

LIVOX_LAUNCH=$WS/SparkCar_Perception/install/livox_ros_driver2/share/livox_ros_driver2/launch_ROS2/msg_MID360_launch.py

# 保存每个后台进程的 PID 数组
PIDS=()

# 关闭所有后台进程
cleanup()
{
  echo
  echo "Stopping SparkCar navigation stack..."

  for pid in "${PIDS[@]}"; do
    if kill -0 "$pid" 2>/dev/null; then
      kill "$pid" 2>/dev/null || true
    fi
  done

  wait 2>/dev/null || true
  echo "All processes stopped."
}

start_bg()
{
  local name="$1"
  shift

  echo
  echo "Starting $name..."
  "$@" &
  local pid=$!
  PIDS+=("$pid")
  echo "$name PID: $pid"
}

trap cleanup EXIT INT TERM

source /opt/ros/humble/setup.bash
source "$WS/SparkCar_Perception/install/setup.bash"
source "$WS/SparkCar_Tools/install/setup.bash"
source "$WS/SparkCar_Navigation/install/setup.bash"

sudo -v

start_bg "Livox MID360" \
  ros2 launch "$LIVOX_LAUNCH"

sleep 5

start_bg "Fast-LIO2 localizer" \
  ros2 launch localizer localizer_launch.py

sleep 3

start_bg "Nav2 bringup" \
  ros2 launch sparkcar_nav_bringup nav_bringup.launch.py

sleep 5

start_bg "Twist to Ackermann converter" \
  ros2 run sparkcar_nav_bringup twist_to_ackermann

sleep 1

start_bg "Micro XRCE-DDS Agent" \
  sudo MicroXRCEAgent serial --dev "$SERIAL_DEV" -b "$BAUDRATE" -v 6

echo
echo "SparkCar navigation stack is running."
echo "Press Enter to stop all processes."
read -r
