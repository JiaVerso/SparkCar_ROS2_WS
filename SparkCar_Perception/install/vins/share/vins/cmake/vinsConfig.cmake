# generated from ament/cmake/core/templates/nameConfig.cmake.in

# prevent multiple inclusion
if(_vins_CONFIG_INCLUDED)
  # ensure to keep the found flag the same
  if(NOT DEFINED vins_FOUND)
    # explicitly set it to FALSE, otherwise CMake will set it to TRUE
    set(vins_FOUND FALSE)
  elseif(NOT vins_FOUND)
    # use separate condition to avoid uninitialized variable warning
    set(vins_FOUND FALSE)
  endif()
  return()
endif()
set(_vins_CONFIG_INCLUDED TRUE)

# output package information
if(NOT vins_FIND_QUIETLY)
  message(STATUS "Found vins: 0.0.0 (${vins_DIR})")
endif()

# warn when using a deprecated package
if(NOT "" STREQUAL "")
  set(_msg "Package 'vins' is deprecated")
  # append custom deprecation text if available
  if(NOT "" STREQUAL "TRUE")
    set(_msg "${_msg} ()")
  endif()
  # optionally quiet the deprecation message
  if(NOT ${vins_DEPRECATED_QUIET})
    message(DEPRECATION "${_msg}")
  endif()
endif()

# flag package as ament-based to distinguish it after being find_package()-ed
set(vins_FOUND_AMENT_PACKAGE TRUE)

# include all config extra files
set(_extras "")
foreach(_extra ${_extras})
  include("${vins_DIR}/${_extra}")
endforeach()
