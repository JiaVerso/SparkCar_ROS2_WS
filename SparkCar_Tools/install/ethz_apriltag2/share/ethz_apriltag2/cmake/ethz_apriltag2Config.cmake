# generated from ament/cmake/core/templates/nameConfig.cmake.in

# prevent multiple inclusion
if(_ethz_apriltag2_CONFIG_INCLUDED)
  # ensure to keep the found flag the same
  if(NOT DEFINED ethz_apriltag2_FOUND)
    # explicitly set it to FALSE, otherwise CMake will set it to TRUE
    set(ethz_apriltag2_FOUND FALSE)
  elseif(NOT ethz_apriltag2_FOUND)
    # use separate condition to avoid uninitialized variable warning
    set(ethz_apriltag2_FOUND FALSE)
  endif()
  return()
endif()
set(_ethz_apriltag2_CONFIG_INCLUDED TRUE)

# output package information
if(NOT ethz_apriltag2_FIND_QUIETLY)
  message(STATUS "Found ethz_apriltag2: 1.0.0 (${ethz_apriltag2_DIR})")
endif()

# warn when using a deprecated package
if(NOT "" STREQUAL "")
  set(_msg "Package 'ethz_apriltag2' is deprecated")
  # append custom deprecation text if available
  if(NOT "" STREQUAL "TRUE")
    set(_msg "${_msg} ()")
  endif()
  # optionally quiet the deprecation message
  if(NOT ${ethz_apriltag2_DEPRECATED_QUIET})
    message(DEPRECATION "${_msg}")
  endif()
endif()

# flag package as ament-based to distinguish it after being find_package()-ed
set(ethz_apriltag2_FOUND_AMENT_PACKAGE TRUE)

# include all config extra files
set(_extras "ament_cmake_export_targets-extras.cmake;ament_cmake_export_dependencies-extras.cmake;ament_cmake_export_include_directories-extras.cmake")
foreach(_extra ${_extras})
  include("${ethz_apriltag2_DIR}/${_extra}")
endforeach()
