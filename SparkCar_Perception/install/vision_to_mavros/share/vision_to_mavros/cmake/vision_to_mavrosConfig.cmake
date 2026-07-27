# generated from ament/cmake/core/templates/nameConfig.cmake.in

# prevent multiple inclusion
if(_vision_to_mavros_CONFIG_INCLUDED)
  # ensure to keep the found flag the same
  if(NOT DEFINED vision_to_mavros_FOUND)
    # explicitly set it to FALSE, otherwise CMake will set it to TRUE
    set(vision_to_mavros_FOUND FALSE)
  elseif(NOT vision_to_mavros_FOUND)
    # use separate condition to avoid uninitialized variable warning
    set(vision_to_mavros_FOUND FALSE)
  endif()
  return()
endif()
set(_vision_to_mavros_CONFIG_INCLUDED TRUE)

# output package information
if(NOT vision_to_mavros_FIND_QUIETLY)
  message(STATUS "Found vision_to_mavros: 0.1.0 (${vision_to_mavros_DIR})")
endif()

# warn when using a deprecated package
if(NOT "" STREQUAL "")
  set(_msg "Package 'vision_to_mavros' is deprecated")
  # append custom deprecation text if available
  if(NOT "" STREQUAL "TRUE")
    set(_msg "${_msg} ()")
  endif()
  # optionally quiet the deprecation message
  if(NOT ${vision_to_mavros_DEPRECATED_QUIET})
    message(DEPRECATION "${_msg}")
  endif()
endif()

# flag package as ament-based to distinguish it after being find_package()-ed
set(vision_to_mavros_FOUND_AMENT_PACKAGE TRUE)

# include all config extra files
set(_extras "")
foreach(_extra ${_extras})
  include("${vision_to_mavros_DIR}/${_extra}")
endforeach()
