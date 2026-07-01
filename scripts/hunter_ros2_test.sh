#!/usr/bin/env bash

# Start Nav2 and the Hunter SE ROS 2 chassis driver.
# Run this script in Ubuntu/ROS 2 Humble, not in Windows PowerShell.

set -e

WS=${WS:-/home/jiaverso/Desktop/SparkCar_ROS2_WS}
CAN_IFACE=${CAN_IFACE:-can0}
CAN_BITRATE=${CAN_BITRATE:-500000}
ROBOT_MODEL=${ROBOT_MODEL:-hunter_se}
ODOM_FRAME=${ODOM_FRAME:-odom}
BASE_FRAME=${BASE_FRAME:-body}
ODOM_TOPIC=${ODOM_TOPIC:-odom}
USE_RVIZ=${USE_RVIZ:-true}

# Set SETUP_CAN=0 to skip CAN setup when can0 is already up.
SETUP_CAN=${SETUP_CAN:-1}

PIDS=()

cleanup()
{
  echo
  echo "Stopping Hunter Nav2 stack..."

  for pid in "${PIDS[@]}"; do
    if kill -0 "$pid" 2>/dev/null; then
      kill "$pid" 2>/dev/null || true
    fi
  done

  wait 2>/dev/null || true
  echo "All Hunter processes stopped."
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
source "$WS/HunterSE_Driver/install/setup.bash"
source "$WS/SparkCar_Navigation/install/setup.bash"

if [[ "$SETUP_CAN" == "1" ]]; then
  echo
  echo "Configuring CAN interface: $CAN_IFACE @ $CAN_BITRATE"
  sudo -v
  sudo modprobe gs_usb || true
  sudo ip link set "$CAN_IFACE" down 2>/dev/null || true
  sudo ip link set "$CAN_IFACE" type can bitrate "$CAN_BITRATE"
  sudo ip link set "$CAN_IFACE" up
fi

echo
echo "CAN interface status:"
ip -details link show "$CAN_IFACE" || true

start_bg "Hunter base driver" \
  ros2 launch hunter_base hunter_base.launch.py \
    port_name:="$CAN_IFACE" \
    robot_model:="$ROBOT_MODEL" \
    odom_frame:="$ODOM_FRAME" \
    base_frame:="$BASE_FRAME" \
    odom_topic_name:="$ODOM_TOPIC"

sleep 3

start_bg "Nav2 bringup" \
  ros2 launch sparkcar_nav_bringup nav_bringup.launch.py \
    use_rviz:="$USE_RVIZ"

echo
echo "Hunter Nav2 stack is running."
echo "Press Enter to stop all processes."
read -r
