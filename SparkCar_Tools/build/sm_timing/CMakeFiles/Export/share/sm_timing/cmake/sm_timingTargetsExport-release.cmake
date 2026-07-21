#----------------------------------------------------------------
# Generated CMake target import file for configuration "Release".
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Import target "sm_timing::sm_timing" for configuration "Release"
set_property(TARGET sm_timing::sm_timing APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(sm_timing::sm_timing PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_RELEASE "CXX"
  IMPORTED_LOCATION_RELEASE "${_IMPORT_PREFIX}/lib/libsm_timing.a"
  )

list(APPEND _IMPORT_CHECK_TARGETS sm_timing::sm_timing )
list(APPEND _IMPORT_CHECK_FILES_FOR_sm_timing::sm_timing "${_IMPORT_PREFIX}/lib/libsm_timing.a" )

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)
