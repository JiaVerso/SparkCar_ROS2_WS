#----------------------------------------------------------------
# Generated CMake target import file for configuration "Release".
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Import target "sm_boost::sm_boost" for configuration "Release"
set_property(TARGET sm_boost::sm_boost APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(sm_boost::sm_boost PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_RELEASE "CXX"
  IMPORTED_LOCATION_RELEASE "${_IMPORT_PREFIX}/lib/libsm_boost.a"
  )

list(APPEND _IMPORT_CHECK_TARGETS sm_boost::sm_boost )
list(APPEND _IMPORT_CHECK_FILES_FOR_sm_boost::sm_boost "${_IMPORT_PREFIX}/lib/libsm_boost.a" )

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)
