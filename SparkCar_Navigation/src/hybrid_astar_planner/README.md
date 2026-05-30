# 安装依赖 ubuntu22.04 ros2 humble
sudo apt update

sudo apt install \
  ros-humble-navigation2 \
  ros-humble-nav2-bringup \
  ros-humble-nav2-map-server \
  ros-humble-nav2-costmap-2d \
  ros-humble-rviz2 \
  ros-humble-tf2 \
  ros-humble-tf2-ros \
  ros-humble-tf2-geometry-msgs \
  ros-humble-robot-state-publisher \
  ros-humble-joint-state-publisher \
  ros-humble-xacro \
  libeigen3-dev \
  libgoogle-glog-dev \
  python3-colcon-common-extensions


  sudo apt install \
  ros-humble-rclcpp \
  ros-humble-nav-msgs \
  ros-humble-geometry-msgs \
  ros-humble-visualization-msgs \
  ros-humble-tf2 \
  ros-humble-tf2-ros \
  ros-humble-tf2-geometry-msgs \
  ros-humble-nav2-map-server \
  ros-humble-rviz2 \
  libeigen3-dev \
  libgoogle-glog-dev \
  python3-colcon-common-extensions

  # 移植注意点 
  ros::NodeHandle              -> rclcpp::Node
ros::Publisher               -> rclcpp::Publisher<T>::SharedPtr
ros::Subscriber              -> rclcpp::Subscription<T>::SharedPtr
ros::Rate                    -> rclcpp::Rate
ros::spinOnce()              -> rclcpp::spin_some()
ros::Time::now()             -> node->now()
tf::getYaw                   -> tf2::getYaw
tf::createQuaternionMsg...   -> tf2::Quaternion + tf2::toMsg
roslaunch                    -> ROS2 Python launch
catkin_make                  -> colcon build
package.xml catkin           -> ament_cmake
CMakeLists.txt catkin        -> ament_cmake 写法

# 保留部分 

src/hybrid_a_star.cpp
src/rs_path.cpp
include/hybrid_a_star/hybrid_a_star.h
include/hybrid_a_star/rs_path.h
include/hybrid_a_star/state_node.h
include/hybrid_a_star/type.h

# ROS 通信层

src/costmap_subscriber.cpp
src/init_pose_subscriber.cpp
src/goal_pose_subscriber.cpp
src/hybrid_a_star_flow.cpp
app/run_hybrid_astar.cpp


##  测试命令

source /opt/ros/humble/setup.bash
source ~/SparkCar_ROS2_WS/SparkCar_Tools/install/setup.bash

ros2 launch pcd2pgm pcd2pgm_launch.py

// 保存pcd点云地图为栅格地图
ros2 run nav2_map_server map_saver_cli -f /home/jiaverso/Desktop/Save_Map/nav2_map
// 查看nav2 生命周期节点
ros2 lifecycle get /map_server
// 确认定位是否正确
ros2 run tf2_ros tf2_echo map body
ros2 lifecycle get /bt_navigator
ros2 lifecycle get /planner_server
ros2 lifecycle get /controller_server
ros2 topic echo /plan
ros2 topic echo /cmd_vel


# 备忘录库

geometry_msgs/msg/Twist 是 ROS2 里的一个标准消息类型，用来表示速度指令。