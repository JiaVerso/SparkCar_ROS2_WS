cmake_minimum_required(VERSION 3.8)
project(target_follower)

if(CMAKE_CXX_COMPILER_ID MATCHES "GNU|Clang")
  add_compile_options(-Wall -Wextra -Wpedantic)
endif()

find_package(ament_cmake REQUIRED)
find_package(rclcpp REQUIRED)
find_package(geometry_msgs REQUIRED)
find_package(std_srvs REQUIRED)

add_library(target_follower_pid src/pid_controller.cpp)
target_compile_features(target_follower_pid PUBLIC cxx_std_17)
target_include_directories(target_follower_pid PUBLIC
  $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
  $<INSTALL_INTERFACE:include>
)

add_executable(target_follower_node src/target_follower_node.cpp)
target_compile_features(target_follower_node PUBLIC cxx_std_17)
ament_target_dependencies(target_follower_node rclcpp geometry_msgs std_srvs)
target_link_libraries(target_follower_node target_follower_pid)

install(TARGETS target_follower_pid target_follower_node
  ARCHIVE DESTINATION lib
  LIBRARY DESTINATION lib
  RUNTIME DESTINATION lib/${PROJECT_NAME}
)

install(DIRECTORY include/
  DESTINATION include
)

install(DIRECTORY launch/
  DESTINATION share/${PROJECT_NAME}/launch
  FILES_MATCHING PATTERN "*.launch.py"
)

install(DIRECTORY config/
  DESTINATION share/${PROJECT_NAME}/config
  FILES_MATCHING PATTERN "*.yaml"
)

install(FILES README.md
  DESTINATION share/${PROJECT_NAME}
)

if(BUILD_TESTING)
  enable_testing()
  add_executable(pid_controller_test test/pid_controller_test.cpp)
  target_compile_features(pid_controller_test PUBLIC cxx_std_17)
  target_link_libraries(pid_controller_test target_follower_pid)
  add_test(NAME pid_controller_test COMMAND pid_controller_test)
endif()

ament_package()
