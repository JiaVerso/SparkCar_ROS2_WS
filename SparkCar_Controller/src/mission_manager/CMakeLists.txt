cmake_minimum_required(VERSION 3.8)
project(mission_manager)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

if(CMAKE_CXX_COMPILER_ID MATCHES "GNU|Clang")
  add_compile_options(-Wall -Wextra -Wpedantic)
endif()

find_package(ament_cmake REQUIRED)
find_package(rclcpp REQUIRED)
find_package(geometry_msgs REQUIRED)
find_package(mavros_msgs REQUIRED)

add_executable(apm_takeoff_node
  src/apm_takeoff_node.cpp
)

target_compile_features(apm_takeoff_node PUBLIC cxx_std_17)

ament_target_dependencies(apm_takeoff_node
  rclcpp
  geometry_msgs
  mavros_msgs
)

install(TARGETS
  apm_takeoff_node
 
  DESTINATION lib/${PROJECT_NAME}
)

# 新添的安装配置文件 
install(DIRECTORY config/
  DESTINATION share/${PROJECT_NAME}/config
  FILES_MATCHING PATTERN "*.yaml"
)

# 新添的启动文件
install(DIRECTORY launch/
  DESTINATION share/${PROJECT_NAME}/launch
  FILES_MATCHING PATTERN "*.launch.py"
)

ament_package()