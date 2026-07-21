# generated from ament/cmake/core/templates/nameConfig.cmake.in

# prevent multiple inclusion
if(_aslam_cv_python_CONFIG_INCLUDED)
  # ensure to keep the found flag the same
  if(NOT DEFINED aslam_cv_python_FOUND)
    # explicitly set it to FALSE, otherwise CMake will set it to TRUE
    set(aslam_cv_python_FOUND FALSE)
  elseif(NOT aslam_cv_python_FOUND)
    # use separate condition to avoid uninitialized variable warning
    set(aslam_cv_python_FOUND FALSE)
  endif()
  return()
endif()
set(_aslam_cv_python_CONFIG_INCLUDED TRUE)

# output package information
if(NOT aslam_cv_python_FIND_QUIETLY)
  message(STATUS "Found aslam_cv_python: 1.0.0 (${aslam_cv_python_DIR})")
endif()

# warn when using a deprecated package
if(NOT "" STREQUAL "")
  set(_msg "Package 'aslam_cv_python' is deprecated")
  # append custom deprecation text if available
  if(NOT "" STREQUAL "TRUE")
    set(_msg "${_msg} ()")
  endif()
  # optionally quiet the deprecation message
  if(NOT ${aslam_cv_python_DEPRECATED_QUIET})
    message(DEPRECATION "${_msg}")
  endif()
endif()

# flag package as ament-based to distinguish it after being find_package()-ed
set(aslam_cv_python_FOUND_AMENT_PACKAGE TRUE)

# include all config extra files
set(_extras "ament_cmake_export_dependencies-extras.cmake;ament_cmake_export_include_directories-extras.cmake")
foreach(_extra ${_extras})
  include("${aslam_cv_python_DIR}/${_extra}")
endforeach()
