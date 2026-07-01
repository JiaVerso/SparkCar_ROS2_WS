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
LIVOX_LAUNCH=${LIVOX_LAUNCH:-$WS/SparkCar_Perception/src/livox_ros_driver2/launch_ROS2/msg_MID360_launch.py}
LOCALIZER_LAUNCH=${LOCALIZER_LAUNCH:-$WS/SparkCar_Perception/src/FASTLIO2_ROS2/localizer/launch/localizer_launch.py}
TF_WAIT_TIMEOUT=${TF_WAIT_TIMEOUT:-45}

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

wait_for_tf()
{
  local target_frame="$1"
  local source_frame="$2"
  local timeout_sec="$3"
  local start_time
  local now
  local tmp_file

  start_time=$(date +%s)
  tmp_file=$(mktemp /tmp/hunter_tf_wait.XXXXXX)

  echo
  echo "Waiting for TF $target_frame -> $source_frame..."
  while true; do
    timeout 2 ros2 run tf2_ros tf2_echo "$target_frame" "$source_frame" > "$tmp_file" 2>&1 || true
    if grep -q "Translation:" "$tmp_file"; then
      echo "TF $target_frame -> $source_frame is available."
      return 0
    fi

    now=$(date +%s)
    if (( now - start_time >= timeout_sec )); then
      echo "Timed out waiting for TF $target_frame -> $source_frame."
      echo "Last TF error:"
      tail -n 5 "$tmp_file" || true
      return 1
    fi
    sleep 1
  done
}

trap cleanup EXIT INT TERM

source /opt/ros/humble/setup.bash
source "$WS/HunterSE_Driver/install/setup.bash"
source "$WS/SparkCar_Perception/install/setup.bash"
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

start_bg "Livox MID360" \
  ros2 launch "$LIVOX_LAUNCH"

sleep 5

start_bg "Fast-LIO2 localizer" \
  ros2 launch "$LOCALIZER_LAUNCH" \
    use_rviz:=false

sleep 3

wait_for_tf map "$BASE_FRAME" "$TF_WAIT_TIMEOUT"

start_bg "Nav2 obstacle cloud filter" \
  ros2 run sparkcar_nav_bringup obstacle_cloud_filter

sleep 1

start_bg "Nav2 bringup" \
  ros2 launch sparkcar_nav_bringup nav_bringup.launch.py \
    use_rviz:="$USE_RVIZ"

echo
echo "Hunter Nav2 stack is running."
echo "Press Enter to stop all processes."
read -r
