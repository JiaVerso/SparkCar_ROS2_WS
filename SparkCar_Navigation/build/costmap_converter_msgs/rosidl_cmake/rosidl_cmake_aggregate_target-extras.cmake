# generated from rosidl_cmake/cmake/rosidl_cmake_aggregate_target-extras.cmake.in

# Create a convenience aggregate target costmap_converter_msgs::costmap_converter_msgs
# that links all generated interface targets, so downstream packages can use
# a single modern CMake target name instead of ${costmap_converter_msgs_TARGETS}.
if(costmap_converter_msgs_TARGETS AND NOT TARGET costmap_converter_msgs::costmap_converter_msgs)
  add_library(costmap_converter_msgs::costmap_converter_msgs INTERFACE IMPORTED)
  set_target_properties(costmap_converter_msgs::costmap_converter_msgs PROPERTIES
    INTERFACE_LINK_LIBRARIES "${costmap_converter_msgs_TARGETS}")
endif()
