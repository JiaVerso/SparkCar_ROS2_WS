#!/bin/bash
# 自主跟随-一键启动脚本 (yolo_follow.sh)

# 定义安全清理与急停函数（捕获 Ctrl + C 信号）
cleanup() {
    echo -e "\n\033[33m[警告] 捕获到 Ctrl+C 信号！正在执行急停与杀死PID...\033[0m"
    
    # 关闭自主跟随
    ros2 service call /target_follower_node/set_enabled std_srvs/srv/SetBool "{data: false}" > /dev/null 2>&1
    
    # 向底盘下发零速度指令
    echo " -> 下发刹车指令 (cmd_vel: 0.0)..."
    ros2 topic pub --once /cmd_vel geometry_msgs/msg/Twist "{linear: {x: 0.0}, angular: {z: 0.0}}" > /dev/null 2>&1
    
    # 杀掉所有后台运行的 Launch 节点和监控进程
    echo " -> 正在终止底层驱动与跟随节点..."
    kill -9 $HUNTER_PID $FOLLOWER_PID $STATUS_MON_PID $VEL_MON_PID > /dev/null 2>&1
    
    echo -e "\033[32m[完毕] 所有服务已安全停止，车体已锁定。\033[0m"
    exit 0
}

# 注册信号监听
trap cleanup SIGINT SIGTERM

# 初始化环境
WORK_DIR="/home/jiaverso/Desktop/SparkCar_ROS2_WS/HunterSE_Driver"
echo -e "\033[36m[Step 1/4] 初始化 ROS 2 Humble 与工作空间环境变量...\033[0m"
cd "$WORK_DIR" || { echo "无法进入目录 $WORK_DIR"; exit 1; }

source /opt/ros/humble/setup.bash
source install/setup.bash


# 后台静默
echo -e "\033[36m[Step 2/4] 正在启动 Hunter SE 底盘 CAN 驱动与 target_follower...\033[0m"

ros2 launch hunter_base hunter_base.launch.py \
  port_name:=can0 \
  robot_model:=hunter_se \
  odom_frame:=wheel_odom \
  base_frame:=wheel_body \
  odom_topic_name:=wheel_odom > /tmp/hunter_base.log 2>&1 &
HUNTER_PID=$!

sleep 3  

# 自主跟随节点
ros2 launch target_follower target_follower.launch.py > /tmp/target_follower.log 2>&1 &
FOLLOWER_PID=$!

sleep 2  

echo -e "\n\033[32m<----------------------------------------------------------->\033[0m"
echo -e "\033[32m  底盘驱动与跟随节点已就绪！\033[0m"
echo -e "\033[33m  请在当前终端按 【回车键 (Enter)】 立即激活自主跟随控制！\033[0m"
echo -e "\033[31m  运行中按 【Ctrl + C】 触发紧急刹车并退出程序。\033[0m"
echo -e "\033[32m<----------------------------------------------------------->\033[0m"

# 阻塞等待用户按回车键
read -r -p ""

echo -e "\033[36m[Step 3/4] 正在调用 SetBool 服务激活跟随...\033[0m"
ros2 service call /target_follower_node/set_enabled std_srvs/srv/SetBool "{data: true}"

echo -e "\033[36m[Step 4/4] 自主跟随已运行！\033[0m"
echo "----------------------------------------------------------------"

# 仅提取控制模式、电量电压、当前线速度/转向角等核心字段
ros2 topic echo /hunter_status | grep --line-buffered -E "control_mode|battery_voltage|linear_velocity|steering_angle" | sed --unbuffered 's/^/🤖 [Hunter 状态] /' &
STATUS_MON_PID=$!

# 仅提取下发的线速度 x 与角速度/转向 z
ros2 topic echo /cmd_vel | grep --line-buffered -E "x:|z:" | sed --unbuffered 's/^/⚡ [跟随下发指令] /' &
VEL_MON_PID=$!

wait