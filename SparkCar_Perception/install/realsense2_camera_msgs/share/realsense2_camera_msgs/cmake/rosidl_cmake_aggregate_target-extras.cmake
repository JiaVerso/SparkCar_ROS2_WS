# generated from rosidl_cmake/cmake/rosidl_cmake_aggregate_target-extras.cmake.in

# Create a convenience aggregate target realsense2_camera_msgs::realsense2_camera_msgs
# that links all generated interface targets, so downstream packages can use
# a single modern CMake target name instead of ${realsense2_camera_msgs_TARGETS}.
if(realsense2_camera_msgs_TARGETS AND NOT TARGET realsense2_camera_msgs::realsense2_camera_msgs)
  add_library(realsense2_camera_msgs::realsense2_camera_msgs INTERFACE IMPORTED)
  set_target_properties(realsense2_camera_msgs::realsense2_camera_msgs PROPERTIES
    INTERFACE_LINK_LIBRARIES "${realsense2_camera_msgs_TARGETS}")
endif()
