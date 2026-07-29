#!/usr/bin/env bash

set -Ee -o pipefail

source /opt/ros/humble/setup.bash

WORKSPACE="/home/orangepi/Desktop/SparkCar_ROS2_WS"

for setup_file in \
  "${WORKSPACE}/SparkCar_Perception/install/setup.bash" \
  "${WORKSPACE}/SparkCar_Controller/install/setup.bash" \
  "${WORKSPACE}/install/setup.bash"
do
  if [[ -f "${setup_file}" ]]; then
    source "${setup_file}"
  fi
done

set -u

LOG_FILE="${TMPDIR:-/tmp}/mavros_$(date +%Y%m%d_%H%M%S)_$$.log"

MAVROS_PID=""
CLEANED_UP=false

process_alive()
{
  local pid="$1"
  kill -0 "${pid}" 2>/dev/null
}

stop_process_group()
{
  local pid="${1:-}"

  if [[ -z "${pid}" ]]; then
    return
  fi

  if ! kill -0 -- "-${pid}" 2>/dev/null; then
    wait "${pid}" 2>/dev/null || true
    return
  fi

  kill -INT -- "-${pid}" 2>/dev/null || true

  for ((i = 0; i < 30; ++i)); do
    if ! kill -0 -- "-${pid}" 2>/dev/null; then
      wait "${pid}" 2>/dev/null || true
      return
    fi

    sleep 0.1
  done

  kill -TERM -- "-${pid}" 2>/dev/null || true
  wait "${pid}" 2>/dev/null || true
}

cleanup()
{
  if [[ "${CLEANED_UP}" == "true" ]]; then
    return
  fi

  CLEANED_UP=true
  set +e

  stop_process_group "${MAVROS_PID}"

  echo "mavros stopped"
}

fail()
{
  local message="$1"

  echo "ERROR: ${message}" >&2

  if [[ -f "${LOG_FILE}" ]]; then
    echo "----- ${LOG_FILE} -----" >&2
    tail -n 30 "${LOG_FILE}" >&2
  fi

  exit 1
}

measure_topic_hz()
{
  local topic="$1"
  local duration="$2"
  local output
  local measured_hz

  output="$(
    timeout "${duration}" \
      ros2 topic hz "${topic}" 2>/dev/null || true
  )"

  measured_hz="$(
    printf '%s\n' "${output}" |
      awk '/average rate:/ {value=$3} END {print value}'
  )"

  if [[ -z "${measured_hz}" ]]; then
    return 1
  fi

  printf '%s' "${measured_hz}"
}

wait_for_service()
{
  local service_name="$1"
  local timeout_seconds="$2"

  for ((i = 0; i < timeout_seconds; ++i)); do
    if ros2 service list 2>/dev/null |
      grep -Fxq "${service_name}"
    then
      return 0
    fi

    if ! process_alive "${MAVROS_PID}"; then
      return 2
    fi

    sleep 1
  done

  return 1
}

trap cleanup EXIT
trap 'exit 130' INT TERM

# 启动MAVROS并隐藏普通输出
setsid ros2 launch mavros apm.launch \
  fcu_url:=serial:///dev/ttyS6:921600 \
  >"${LOG_FILE}" 2>&1 &

MAVROS_PID=$!

# 等待飞控心跳
FCU_CONNECTED=false

for ((i = 0; i < 60; ++i)); do
  state="$(
    timeout 2 ros2 topic echo \
      /mavros/state \
      --once \
      --qos-reliability best_effort \
      2>/dev/null || true
  )"

  if grep -q "connected: true" <<<"${state}"; then
    FCU_CONNECTED=true
    break
  fi

  if ! process_alive "${MAVROS_PID}"; then
    fail "MAVROS exited while waiting for FCU"
  fi

  sleep 1
done

if [[ "${FCU_CONNECTED}" != "true" ]]; then
  fail "timeout waiting for FCU connection"
fi

echo "apm connection"

# 优先使用按消息ID设置频率
if wait_for_service "/mavros/set_message_interval" 10; then
  timeout 10 ros2 service call \
    /mavros/set_message_interval \
    mavros_msgs/srv/MessageInterval \
    "{message_id: 27, message_rate: 200.0}" \
    >/dev/null 2>&1 || true

  echo "RAW_IMU requested: 200 Hz"
else
  # 旧式SET_DATA_STREAM回退方案
  if ! wait_for_service "/mavros/set_stream_rate" 20; then
    fail "no MAVROS stream-rate service is available"
  fi

  timeout 10 ros2 service call \
    /mavros/set_stream_rate \
    mavros_msgs/srv/StreamRate \
    "{stream_id: 1, message_rate: 200, on_off: true}" \
    >/dev/null 2>&1 || true

  echo "RAW_SENSORS stream requested: 200 Hz"
fi

# 测量实际IMU频率
imu_hz="$(measure_topic_hz "/mavros/imu/data_raw" 12 || true)"

if [[ -z "${imu_hz}" ]]; then
  ros2 topic info /mavros/imu/data_raw -v >&2 || true
  fail "unable to receive /mavros/imu/data_raw"
fi

echo "imu measured: ${imu_hz} Hz"
echo "log: ${LOG_FILE}"
echo "press Ctrl+C to stop MAVROS"

# 保持MAVROS运行
while true; do
  if ! process_alive "${MAVROS_PID}"; then
    fail "MAVROS exited unexpectedly"
  fi

  sleep 1
done