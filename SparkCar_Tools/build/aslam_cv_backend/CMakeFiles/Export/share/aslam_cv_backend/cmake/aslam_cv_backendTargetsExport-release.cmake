#----------------------------------------------------------------
# Generated CMake target import file for configuration "Release".
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Import target "aslam_cv_backend::aslam_cv_backend" for configuration "Release"
set_property(TARGET aslam_cv_backend::aslam_cv_backend APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(aslam_cv_backend::aslam_cv_backend PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_RELEASE "CXX"
  IMPORTED_LOCATION_RELEASE "${_IMPORT_PREFIX}/lib/libaslam_cv_backend.a"
  )

list(APPEND _IMPORT_CHECK_TARGETS aslam_cv_backend::aslam_cv_backend )
list(APPEND _IMPORT_CHECK_FILES_FOR_aslam_cv_backend::aslam_cv_backend "${_IMPORT_PREFIX}/lib/libaslam_cv_backend.a" )

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)
