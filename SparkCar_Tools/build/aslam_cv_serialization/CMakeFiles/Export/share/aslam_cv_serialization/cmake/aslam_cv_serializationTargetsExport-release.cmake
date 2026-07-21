#----------------------------------------------------------------
# Generated CMake target import file for configuration "Release".
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Import target "aslam_cv_serialization::aslam_cv_serialization" for configuration "Release"
set_property(TARGET aslam_cv_serialization::aslam_cv_serialization APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(aslam_cv_serialization::aslam_cv_serialization PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_RELEASE "CXX"
  IMPORTED_LOCATION_RELEASE "${_IMPORT_PREFIX}/lib/libaslam_cv_serialization.a"
  )

list(APPEND _IMPORT_CHECK_TARGETS aslam_cv_serialization::aslam_cv_serialization )
list(APPEND _IMPORT_CHECK_FILES_FOR_aslam_cv_serialization::aslam_cv_serialization "${_IMPORT_PREFIX}/lib/libaslam_cv_serialization.a" )

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)
