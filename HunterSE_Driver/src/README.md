# Hunter SE 底盘驱动操作文档

本文档说明 `HunterSE_Driver/src` 目录的作用、Hunter SE 底盘驱动启动方法、基础运动测试方法，以及接入 Nav2 自动导航时需要注意的适配事项。

## 目录作用

```text
HunterSE_Driver/src
├── hunter_ros2
│   ├── hunter_base      # ROS 2 底盘驱动节点
│   └── hunter_msgs      # Hunter 状态消息定义
└── ugv_sdk              # AgileX / Weston Robot UGV SDK，负责 CAN 底层通信
```

`hunter_base` 是 ROS 2 和 Hunter 底盘之间的桥接层。它通过 `ugv_sdk` 连接 CAN 总线，订阅 `/cmd_vel`，把 `geometry_msgs/msg/Twist` 中的 `linear.x` 和 `angular.z` 换算成 Hunter 底盘的速度与转向角命令。

运行后主要提供这些接口：

- 订阅：`/cmd_vel`
- 发布：`/odom`
- 发布 TF：`odom -> base_link`
- 发布状态：`/hunter_status`

所以它本身不是键盘控制节点，也不是 Nav2；它只是底盘驱动。键盘控制、Nav2 控制、脚本控制都需要通过发布 `/cmd_vel` 来驱动车辆。

## 环境准备

推荐环境：

- Ubuntu 22.04
- ROS 2 Humble
- CAN 转 USB 或板载 CAN
- Hunter SE 底盘已上电，急停释放，遥控器可随时接管

常用依赖：

```bash
sudo apt update
sudo apt install -y \
  python3-colcon-common-extensions \
  ros-humble-rclcpp \
  ros-humble-geometry-msgs \
  ros-humble-nav-msgs \
  ros-humble-sensor-msgs \
  ros-humble-tf2 \
  ros-humble-tf2-ros \
  ros-humble-tf2-geometry-msgs \
  ros-humble-teleop-twist-keyboard \
  can-utils \
  libasio-dev \
  libeigen3-dev \
  libgoogle-glog-dev
```

## CAN 通信配置

Hunter 底盘默认使用 SocketCAN，launch 默认参数是 `port_name:=can0`。

加载 CAN USB 驱动：

```bash
sudo modprobe gs_usb
```

手动配置 `can0`，波特率 500 kbit/s：

```bash
sudo ip link set can0 down
sudo ip link set can0 type can bitrate 500000
sudo ip link set can0 up
```

检查 CAN 状态：

```bash
ip -details link show can0
```

检查是否有底盘 CAN 数据：

```bash
candump can0
```

如果 `candump can0` 完全没有数据，优先检查 CAN-H/CAN-L 接线、终端电阻、底盘供电、急停、CAN 设备名是否真的是 `can0`。

## 编译驱动

从 HunterSE_Driver 工作区编译：

```bash
cd /home/jiaverso/Desktop/SparkCar_ROS2_WS/HunterSE_Driver
source /opt/ros/humble/setup.bash
colcon build --symlink-install
```

编译完成后加载环境：

```bash
source /opt/ros/humble/setup.bash
source /home/jiaverso/Desktop/SparkCar_ROS2_WS/HunterSE_Driver/install/setup.bash
```

可以检查包是否可见：

```bash
ros2 pkg list | grep hunter
ros2 pkg executables hunter_base
```

## 启动 Hunter SE 底盘驱动

启动实车底盘：

```bash
source /opt/ros/humble/setup.bash
source /home/jiaverso/Desktop/SparkCar_ROS2_WS/HunterSE_Driver/install/setup.bash

ros2 launch hunter_base hunter_base.launch.py port_name:=can0 robot_model:=hunter_se
```

常用 launch 参数：

| 参数 | 默认值 | 说明 |
| --- | --- | --- |
| `port_name` | `can0` | CAN 设备名 |
| `robot_model` | `hunter2` | 建议 Hunter SE 使用 `hunter_se` |
| `odom_frame` | `odom` | 里程计坐标系 |
| `base_frame` | `base_link` | 车体坐标系 |
| `odom_topic_name` | `odom` | 里程计话题名 |
| `use_sim_time` | `false` | 是否使用仿真时间 |

如果你的 Nav2 配置使用 `body` 作为机器人坐标系，可以直接把驱动的底盘 frame 改成 `body`：

```bash
ros2 launch hunter_base hunter_base.launch.py \
  port_name:=can0 \
  robot_model:=hunter_se \
  base_frame:=body
```

这样可以避免同时存在 `base_link` 和 `body` 但没有静态 TF 的问题。

## 基础运动测试

### 方式一：直接发布 `/cmd_vel`

低速前进：

```bash
ros2 topic pub /cmd_vel geometry_msgs/msg/Twist \
  "{linear: {x: 0.2}, angular: {z: 0.0}}" -r 10
```

低速后退：

```bash
ros2 topic pub /cmd_vel geometry_msgs/msg/Twist \
  "{linear: {x: -0.2}, angular: {z: 0.0}}" -r 10
```

前进左转：

```bash
ros2 topic pub /cmd_vel geometry_msgs/msg/Twist \
  "{linear: {x: 0.2}, angular: {z: 0.3}}" -r 10
```

停止：

```bash
ros2 topic pub /cmd_vel geometry_msgs/msg/Twist \
  "{linear: {x: 0.0}, angular: {z: 0.0}}" -1
```

### 方式二：键盘控制

底盘驱动不自带键盘控制，需要另开终端运行：

```bash
source /opt/ros/humble/setup.bash
source /home/jiaverso/Desktop/SparkCar_ROS2_WS/HunterSE_Driver/install/setup.bash

ros2 run teleop_twist_keyboard teleop_twist_keyboard
```

常用键位：

```text
u    i    o
j    k    l
m    ,    .
```

| 按键 | 作用 |
| --- | --- |
| `i` | 前进 |
| `,` | 后退 |
| `u` | 前进左转 |
| `o` | 前进右转 |
| `m` | 后退左转 |
| `.` | 后退右转 |
| `k` | 停止 |
| `q` / `z` | 同时增大 / 减小线速度和角速度 |
| `w` / `x` | 增大 / 减小线速度 |
| `e` / `c` | 增大 / 减小角速度 |

Hunter SE 是阿克曼转向底盘，不能像差速车一样原地旋转。`j` 和 `l` 这类只有角速度、没有线速度的命令不适合作为主要测试方式；建议使用 `u`、`o`、`m`、`.` 这类“边走边转”的命令。

## 状态检查

查看话题：

```bash
ros2 topic list
```

查看底盘状态：

```bash
ros2 topic echo /hunter_status
```

查看里程计：

```bash
ros2 topic echo /odom
ros2 topic hz /odom
```

查看 `/cmd_vel` 是否有控制命令：

```bash
ros2 topic echo /cmd_vel
```

查看 TF：

```bash
ros2 run tf2_ros tf2_echo odom base_link
```

如果启动时使用了 `base_frame:=body`，则检查：

```bash
ros2 run tf2_ros tf2_echo odom body
```

## Nav2 导航适配事项

### 1. `/cmd_vel` 可以直接接 Hunter 驱动

Nav2 的 controller_server 默认输出 `geometry_msgs/msg/Twist` 到 `/cmd_vel`。当前 Hunter 驱动也订阅 `/cmd_vel`，因此可以直接使用：

```text
Nav2 controller_server -> /cmd_vel -> hunter_base -> CAN -> Hunter SE
```

如果使用当前 `hunter_base` 驱动，不需要再运行 `twist_to_ackermann` 转换节点，因为该转换节点发布的是 `/ackermann_cmd`，而 `hunter_base` 并不订阅 `/ackermann_cmd`。

只有当你的底层控制器明确订阅 `/ackermann_cmd` 时，才需要：

```bash
ros2 run sparkcar_nav_bringup twist_to_ackermann
```

### 2. Nav2 要按阿克曼车配置

Hunter SE 不是差速车，不能原地转向。Nav2 中应避免使用需要原地旋转能力的配置。建议：

- 全局规划器使用 `nav2_smac_planner/SmacPlannerHybrid`
- `motion_model_for_search` 使用 `REEDS_SHEPP` 或 `DUBIN`
- 设置合理的 `minimum_turning_radius`
- 局部控制器使用 `RegulatedPurePursuitController`
- 关闭或弱化原地旋转到目标朝向的行为
- 目标点 `yaw_goal_tolerance` 不要过小

当前仓库的 Nav2 配置文件在：

```text
/home/jiaverso/Desktop/SparkCar_ROS2_WS/SparkCar_Navigation/src/sparkcar_nav_bringup/config/nav2_params.yaml
```

其中已经使用了 `SmacPlannerHybrid` 和 `RegulatedPurePursuitController`，整体方向适合阿克曼底盘。

Hunter SE 参数中给出的轴距约为 `0.52 m`，最大转角约为 `0.283 rad`。理论最小转弯半径可按下面估算：

```text
R = wheelbase / tan(max_steering_angle)
R = 0.52 / tan(0.283) ≈ 1.79 m
```

因此 Nav2 的 `minimum_turning_radius` 建议不要配置得过小。若设成 `0.4` 这类明显小于实车能力的值，规划路径可能包含实车无法跟踪的小半径转弯。

### 3. TF frame 必须统一

Hunter 驱动默认发布：

```text
odom -> base_link
```

如果 Nav2 配置中 `robot_base_frame` 是 `body`，则必须满足下面任一条件：

- 启动 Hunter 驱动时使用 `base_frame:=body`
- 或发布静态 TF：`base_link -> body`
- 或把 Nav2 配置中的 `robot_base_frame` 改成 `base_link`

推荐简单做法：

```bash
ros2 launch hunter_base hunter_base.launch.py \
  port_name:=can0 \
  robot_model:=hunter_se \
  base_frame:=body
```

然后检查：

```bash
ros2 run tf2_ros tf2_echo odom body
ros2 run tf2_ros tf2_echo map body
```

完整导航时常见 TF 链路应类似：

```text
map -> odom/lidar/localization_frame -> body -> lidar
```

具体中间 frame 取决于定位模块。核心原则是 Nav2 必须能查到 `global_frame` 到 `robot_base_frame` 的变换。

### 4. 避免重复发布同一段 TF

Hunter 驱动会发布里程计 TF。如果 FAST-LIO2、本地化节点或其他节点也发布 `odom -> body`、`odom -> base_link`，可能造成 TF 冲突。冲突表现包括：

- RViz 中机器人跳动
- Nav2 报 transform timeout
- 局部代价地图位置漂移
- controller_server 规划正常但跟踪失败

接入 Nav2 前建议确认谁负责发布以下 TF：

| TF | 推荐来源 |
| --- | --- |
| `map -> odom` 或 `map -> lidar` | 定位模块 |
| `odom -> body/base_link` | 底盘里程计或定位模块二选一 |
| `body/base_link -> lidar` | 静态外参 |

不要让两个节点同时发布同一对 parent/child frame。

### 5. Nav2 启动顺序建议

如果使用 Hunter 驱动直接接 `/cmd_vel`，推荐启动顺序：

```bash
# 1. 启动底盘
source /opt/ros/humble/setup.bash
source /home/jiaverso/Desktop/SparkCar_ROS2_WS/HunterSE_Driver/install/setup.bash
ros2 launch hunter_base hunter_base.launch.py port_name:=can0 robot_model:=hunter_se base_frame:=body

# 2. 启动雷达和定位
source /home/jiaverso/Desktop/SparkCar_ROS2_WS/SparkCar_Perception/install/setup.bash
ros2 launch livox_ros_driver2 msg_MID360_launch.py
ros2 launch localizer localizer_launch.py

# 3. 启动 Nav2
source /home/jiaverso/Desktop/SparkCar_ROS2_WS/SparkCar_Navigation/install/setup.bash
ros2 launch sparkcar_nav_bringup nav_bringup.launch.py
```

启动后检查：

```bash
ros2 topic echo /cmd_vel
ros2 topic echo /odom
ros2 topic echo /hunter_status
ros2 lifecycle get /controller_server
ros2 lifecycle get /bt_navigator
ros2 run tf2_ros tf2_echo map body
```

## 已知代码注意点

当前 `hunter_base` 源码中，`robot_model:=hunter_se` 分支实际使用的是 `HunterV1Params`，而不是文件中已经定义好的 `HunterSEParams`。这会影响轴距、轮距、最大转角和 `/cmd_vel` 到转向角的换算精度。

相关文件：

```text
hunter_ros2/hunter_base/src/hunter_base_ros.cpp
hunter_ros2/hunter_base/include/hunter_base/hunter_params.hpp
```

建议后续将 `hunter_se` 分支改为使用 `HunterSEParams`，并同步检查 Nav2 中的 `minimum_turning_radius`、`wheelbase`、最大速度和最大角速度限制。

## 常见问题

### 启动后小车不动

检查顺序：

```bash
ip -details link show can0
candump can0
ros2 topic echo /cmd_vel
ros2 topic echo /hunter_status
```

常见原因：

- CAN 设备名不是 `can0`
- CAN 波特率不是 500 kbit/s
- 急停未释放
- 遥控器或底盘未进入可控制模式
- `/cmd_vel` 没有发布
- 线速度太小，被底盘或控制器忽略

### Nav2 有路径但车不走

检查：

```bash
ros2 lifecycle get /controller_server
ros2 topic echo /cmd_vel
ros2 run tf2_ros tf2_echo map body
```

如果 `/cmd_vel` 没有输出，多半是 Nav2 lifecycle、TF、costmap 或 goal checker 问题。如果 `/cmd_vel` 有输出但车不动，再检查 Hunter 驱动、CAN 和 `/hunter_status`。

### 车能走但转弯很差

重点检查：

- `robot_model:=hunter_se` 参数是否设置
- Hunter SE 参数是否使用了真实轴距和最大转角
- Nav2 `minimum_turning_radius` 是否过小
- `desired_linear_vel` 是否过高
- `lookahead_dist` 是否过大或过小
- TF 是否跳变或延迟

### 车到目标点附近反复调整

Hunter SE 不能原地旋转，目标朝向容忍度不能按差速车设置得太严格。建议适当增大：

- `xy_goal_tolerance`
- `yaw_goal_tolerance`

并减少需要原地旋转的行为树动作或恢复行为。

## 安全建议

- 第一次测试时架空车轮，或在空旷区域低速测试。
- 保持遥控器、急停和停止命令可用。
- 初始速度建议 `0.1` 到 `0.2 m/s`。
- 每次运行 Nav2 前先单独测试 `/cmd_vel` 是否能正确让底盘前进、后退、转弯和停止。
- 自动导航前确认 TF 和地图方向正确，避免车辆朝错误方向运动。
