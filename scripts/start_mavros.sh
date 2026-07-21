#!/bin/bash

source /opt/ros/humble/setup.bash

ros2 launch mavros apm.launch \
  fcu_url:=serial:///dev/ttyS6:921600 &
MAVROS_PID=$!

sleep 8

ros2 service call /mavros/set_stream_rate mavros_msgs/srv/StreamRate \
"{stream_id: 1, message_rate: 200, on_off: true}"

wait $MAVROS_PID
