#----------------------------------------------------------------
# Generated CMake target import file for configuration "Release".
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Import target "sm_eigen::sm_eigen" for configuration "Release"
set_property(TARGET sm_eigen::sm_eigen APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(sm_eigen::sm_eigen PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_RELEASE "CXX"
  IMPORTED_LOCATION_RELEASE "${_IMPORT_PREFIX}/lib/libsm_eigen.a"
  )

list(APPEND _IMPORT_CHECK_TARGETS sm_eigen::sm_eigen )
list(APPEND _IMPORT_CHECK_FILES_FOR_sm_eigen::sm_eigen "${_IMPORT_PREFIX}/lib/libsm_eigen.a" )

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)
