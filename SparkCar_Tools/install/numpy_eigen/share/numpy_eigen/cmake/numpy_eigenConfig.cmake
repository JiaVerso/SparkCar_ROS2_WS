# generated from ament/cmake/core/templates/nameConfig.cmake.in

# prevent multiple inclusion
if(_numpy_eigen_CONFIG_INCLUDED)
  # ensure to keep the found flag the same
  if(NOT DEFINED numpy_eigen_FOUND)
    # explicitly set it to FALSE, otherwise CMake will set it to TRUE
    set(numpy_eigen_FOUND FALSE)
  elseif(NOT numpy_eigen_FOUND)
    # use separate condition to avoid uninitialized variable warning
    set(numpy_eigen_FOUND FALSE)
  endif()
  return()
endif()
set(_numpy_eigen_CONFIG_INCLUDED TRUE)

# output package information
if(NOT numpy_eigen_FIND_QUIETLY)
  message(STATUS "Found numpy_eigen: 1.0.0 (${numpy_eigen_DIR})")
endif()

# warn when using a deprecated package
if(NOT "" STREQUAL "")
  set(_msg "Package 'numpy_eigen' is deprecated")
  # append custom deprecation text if available
  if(NOT "" STREQUAL "TRUE")
    set(_msg "${_msg} ()")
  endif()
  # optionally quiet the deprecation message
  if(NOT ${numpy_eigen_DEPRECATED_QUIET})
    message(DEPRECATION "${_msg}")
  endif()
endif()

# flag package as ament-based to distinguish it after being find_package()-ed
set(numpy_eigen_FOUND_AMENT_PACKAGE TRUE)

# include all config extra files
set(_extras "ament_cmake_export_dependencies-extras.cmake;ament_cmake_export_include_directories-extras.cmake")
foreach(_extra ${_extras})
  include("${numpy_eigen_DIR}/${_extra}")
endforeach()
