cmake_minimum_required(VERSION 3.8)
project(vision_to_mavros)

if(CMAKE_COMPILER_IS_GNUCXX OR CMAKE_CXX_COMPILER_ID MATCHES "Clang")
  add_compile_options(-Wall -Wextra -Wpedantic)
endif()

find_package(ament_cmake REQUIRED)
find_package(geometry_msgs REQUIRED)
find_package(nav_msgs REQUIRED)
find_package(rclcpp REQUIRED)
find_package(std_msgs REQUIRED)
find_package(std_srvs REQUIRED)
find_package(tf2 REQUIRED)

add_executable(vision_to_mavros_node src/vision_to_mavros_node.cpp)
target_compile_features(vision_to_mavros_node PUBLIC cxx_std_17)
ament_target_dependencies(
  vision_to_mavros_node
  geometry_msgs
  nav_msgs
  rclcpp
  std_msgs
  std_srvs
  tf2
)

install(TARGETS vision_to_mavros_node
  DESTINATION lib/${PROJECT_NAME})

install(DIRECTORY launch config
  DESTINATION share/${PROJECT_NAME})

ament_package()
