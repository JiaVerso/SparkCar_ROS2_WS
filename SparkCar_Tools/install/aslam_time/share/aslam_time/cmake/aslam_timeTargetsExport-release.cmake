#----------------------------------------------------------------
# Generated CMake target import file for configuration "Release".
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Import target "aslam_time::aslam_time" for configuration "Release"
set_property(TARGET aslam_time::aslam_time APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(aslam_time::aslam_time PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_RELEASE "CXX"
  IMPORTED_LOCATION_RELEASE "${_IMPORT_PREFIX}/lib/libaslam_time.a"
  )

list(APPEND _IMPORT_CHECK_TARGETS aslam_time::aslam_time )
list(APPEND _IMPORT_CHECK_FILES_FOR_aslam_time::aslam_time "${_IMPORT_PREFIX}/lib/libaslam_time.a" )

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)
