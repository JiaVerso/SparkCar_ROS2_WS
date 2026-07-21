#----------------------------------------------------------------
# Generated CMake target import file for configuration "Release".
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Import target "aslam_cameras_april::aslam_cameras_april" for configuration "Release"
set_property(TARGET aslam_cameras_april::aslam_cameras_april APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(aslam_cameras_april::aslam_cameras_april PROPERTIES
  IMPORTED_LOCATION_RELEASE "${_IMPORT_PREFIX}/lib/libaslam_cameras_april.so"
  IMPORTED_SONAME_RELEASE "libaslam_cameras_april.so"
  )

list(APPEND _IMPORT_CHECK_TARGETS aslam_cameras_april::aslam_cameras_april )
list(APPEND _IMPORT_CHECK_FILES_FOR_aslam_cameras_april::aslam_cameras_april "${_IMPORT_PREFIX}/lib/libaslam_cameras_april.so" )

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)
