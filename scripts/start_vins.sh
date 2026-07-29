#!/bin/bash

# ==========================================
# VINS-Fusion -> MAVROS 完整视觉融合启动脚本
# ==========================================

# 1. 刷新工作空间环境变量 (请根据你的实际 ROS 2 版本修改，这里假设为 humble)
source /opt/ros/humble/setup.bash
# 激活你自己的工作空间
source /home/orangepi/Desktop/SparkCar_ROS2_WS/install/setup.bash

echo "=========================================="
echo "  正在启动 RealSense D435 双目红外节点... "
echo "=========================================="
ros2 launch realsense2_camera rs_launch.py \
    enable_color:=false \
    enable_depth:=false \
    enable_infra1:=true \
    enable_infra2:=true \
    depth_module.emitter_enabled:=0 \
    depth_module.profile:=640x480x30 \
    depth_module.infra_profile:=640x480x30 \
    depth_module.infra1_profile:=640x480x30 \
    depth_module.infra2_profile:=640x480x30 &
PID_RS=$!

# 延时 3 秒，等待相机节点完全拉起
sleep 3


echo "=========================================="
echo "  正在启动 VINS-Fusion RViz 可视化界面... "
echo "=========================================="
ros2 launch vins vins_rviz.launch.py &
PID_RVIZ=$!

# 延时 2 秒，错峰启动
sleep 2


echo "=========================================="
echo "  正在启动 VINS-Fusion 核心解算节点...    "
echo "  (终端输出已屏蔽，后台静默运行中)        "
echo "=========================================="
# 使用 > /dev/null 2>&1 将输出重定向到黑洞，保持终端清爽
ros2 launch vins euroc.launch.py \
    config_path:=/home/orangepi/Desktop/SparkCar_ROS2_WS/SparkCar_Perception/src/VINS-Fusion-ROS2/config/realsense_d435/d435_stereo_h743_vins_fusion.yaml > /dev/null 2>&1 &
PID_VINS=$!

# 延时 2 秒，等待 VINS 节点准备好发布话题
sleep 2


echo "=========================================="
echo "  正在启动 Vision to MAVROS 位姿转发节点.."
echo "=========================================="
ros2 launch vision_to_mavros vision_to_mavros.launch.py &
PID_V2M=$!

echo "=========================================="
echo "  所有节点已在后台启动完成！              "
echo "  按 Ctrl+C 可以安全终止所有节点。        "
echo "=========================================="

# 捕获 Ctrl+C (SIGINT) 或终止信号 (SIGTERM)，确保干净退出所有的 PID
trap "echo -e '\n正在关闭所有节点...'; kill $PID_RS $PID_RVIZ $PID_VINS $PID_V2M; wait; echo '已安全退出。'; exit 0" SIGINT SIGTERM

# 挂起脚本，保持前台运行状态以监听 Ctrl+C
wait