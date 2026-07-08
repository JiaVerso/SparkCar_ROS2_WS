# generated from rosidl_cmake/cmake/rosidl_cmake_aggregate_target-extras.cmake.in

# Create a convenience aggregate target teb_msgs::teb_msgs
# that links all generated interface targets, so downstream packages can use
# a single modern CMake target name instead of ${teb_msgs_TARGETS}.
if(teb_msgs_TARGETS AND NOT TARGET teb_msgs::teb_msgs)
  add_library(teb_msgs::teb_msgs INTERFACE IMPORTED)
  set_target_properties(teb_msgs::teb_msgs PROPERTIES
    INTERFACE_LINK_LIBRARIES "${teb_msgs_TARGETS}")
endif()
