#!/usr/bin/env bash



source /opt/ros/humble/setup.bash

ros2 launch mavros apm.launch \
  fcu_url:=serial:///dev/ttyS6:921600 &

MAVROS_PID=$!

cleanup()
{
    kill "$MAVROS_PID" 2>/dev/null || true
}

trap cleanup INT TERM EXIT

echo "Waiting for MAVROS set_stream_rate service..."

SERVICE_READY=false

for i in $(seq 1 90); do
    if ros2 service list 2>/dev/null |
        grep -qx "/mavros/set_stream_rate"; then
        SERVICE_READY=true
        echo "set_stream_rate service is ready."
        break
    fi

    if ! kill -0 "$MAVROS_PID" 2>/dev/null; then
        echo "MAVROS exited during initialization."
        exit 1
    fi

    sleep 1
done

if [ "$SERVICE_READY" != "true" ]; then
    echo "Timeout waiting for /mavros/set_stream_rate."
    exit 1
fi

echo "Waiting for FCU heartbeat..."

FCU_CONNECTED=false

for i in $(seq 1 60); do
    STATE=$(
        timeout 2 ros2 topic echo \
          /mavros/state \
          --once 2>/dev/null || true
    )

    if echo "$STATE" | grep -q "connected: true"; then
        FCU_CONNECTED=true
        echo "FCU connected."
        break
    fi

    sleep 1
done

if [ "$FCU_CONNECTED" != "true" ]; then
    echo "Timeout waiting for FCU connection."
    exit 1
fi

ros2 service call \
  /mavros/set_stream_rate \
  mavros_msgs/srv/StreamRate \
  "{stream_id: 1, message_rate: 200, on_off: true}"

echo "IMU stream requested."

wait "$MAVROS_PID"
