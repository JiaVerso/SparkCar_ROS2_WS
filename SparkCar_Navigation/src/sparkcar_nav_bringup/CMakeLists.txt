cmake_minimum_required(VERSION 3.8)
project(sparkcar_nav_bringup)

if(CMAKE_COMPILER_IS_GNUCXX OR CMAKE_CXX_COMPILER_ID MATCHES "Clang")
  add_compile_options(-Wall -Wextra -Wpedantic)
endif()

# find dependencies
find_package(ament_cmake REQUIRED)
# find_package(nav2_bringup REQUIRED)
find_package(nav2_map_server REQUIRED)
find_package(tf2_ros REQUIRED)

find_package(rclcpp REQUIRED)
find_package(geometry_msgs REQUIRED)
find_package(ackermann_msgs REQUIRED)
find_package(sensor_msgs REQUIRED)

add_executable(twist_to_ackermann src/twist_to_ackermann.cpp)
add_executable(obstacle_cloud_filter src/obstacle_cloud_filter.cpp)

# 把节点和依赖库联系起来
ament_target_dependencies(twist_to_ackermann
  rclcpp
  geometry_msgs
  ackermann_msgs
)

ament_target_dependencies(obstacle_cloud_filter
  rclcpp
  sensor_msgs
)

# 安装节点
install(TARGETS
  twist_to_ackermann
  obstacle_cloud_filter
  DESTINATION lib/${PROJECT_NAME}
)

# 安装工作路径到 share 目录下，方便 launch 文件和配置文件的访问
install(DIRECTORY
  launch
  behavior_trees
  config
  maps
  rviz
  DESTINATION share/${PROJECT_NAME}
)

if(BUILD_TESTING)
  find_package(ament_lint_auto REQUIRED)
  # the following line skips the linter which checks for copyrights
  # comment the line when a copyright and license is added to all source files
  set(ament_cmake_copyright_FOUND TRUE)
  # the following line skips cpplint (only works in a git repo)
  # comment the line when this package is in a git repo and when
  # a copyright and license is added to all source files
  set(ament_cmake_cpplint_FOUND TRUE)
  ament_lint_auto_find_test_dependencies()
endif()

ament_package()
