#----------------------------------------------------------------
# Generated CMake target import file for configuration "Release".
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Import target "aslam_backend_expressions::aslam_backend_expressions" for configuration "Release"
set_property(TARGET aslam_backend_expressions::aslam_backend_expressions APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(aslam_backend_expressions::aslam_backend_expressions PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_RELEASE "CXX"
  IMPORTED_LOCATION_RELEASE "${_IMPORT_PREFIX}/lib/libaslam_backend_expressions.a"
  )

list(APPEND _IMPORT_CHECK_TARGETS aslam_backend_expressions::aslam_backend_expressions )
list(APPEND _IMPORT_CHECK_FILES_FOR_aslam_backend_expressions::aslam_backend_expressions "${_IMPORT_PREFIX}/lib/libaslam_backend_expressions.a" )

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)
