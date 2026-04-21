# generated from ament/cmake/core/templates/nameConfig.cmake.in

# prevent multiple inclusion
if(_hba_CONFIG_INCLUDED)
  # ensure to keep the found flag the same
  if(NOT DEFINED hba_FOUND)
    # explicitly set it to FALSE, otherwise CMake will set it to TRUE
    set(hba_FOUND FALSE)
  elseif(NOT hba_FOUND)
    # use separate condition to avoid uninitialized variable warning
    set(hba_FOUND FALSE)
  endif()
  return()
endif()
set(_hba_CONFIG_INCLUDED TRUE)

# output package information
if(NOT hba_FIND_QUIETLY)
  message(STATUS "Found hba: 0.0.0 (${hba_DIR})")
endif()

# warn when using a deprecated package
if(NOT "" STREQUAL "")
  set(_msg "Package 'hba' is deprecated")
  # append custom deprecation text if available
  if(NOT "" STREQUAL "TRUE")
    set(_msg "${_msg} ()")
  endif()
  # optionally quiet the deprecation message
  if(NOT ${hba_DEPRECATED_QUIET})
    message(DEPRECATION "${_msg}")
  endif()
endif()

# flag package as ament-based to distinguish it after being find_package()-ed
set(hba_FOUND_AMENT_PACKAGE TRUE)

# include all config extra files
set(_extras "")
foreach(_extra ${_extras})
  include("${hba_DIR}/${_extra}")
endforeach()
