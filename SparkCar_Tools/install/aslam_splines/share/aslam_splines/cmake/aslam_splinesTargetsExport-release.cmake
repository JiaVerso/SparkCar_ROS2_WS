#----------------------------------------------------------------
# Generated CMake target import file for configuration "Release".
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Import target "aslam_splines::aslam_splines" for configuration "Release"
set_property(TARGET aslam_splines::aslam_splines APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(aslam_splines::aslam_splines PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_RELEASE "CXX"
  IMPORTED_LOCATION_RELEASE "${_IMPORT_PREFIX}/lib/libaslam_splines.a"
  )

list(APPEND _IMPORT_CHECK_TARGETS aslam_splines::aslam_splines )
list(APPEND _IMPORT_CHECK_FILES_FOR_aslam_splines::aslam_splines "${_IMPORT_PREFIX}/lib/libaslam_splines.a" )

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)
