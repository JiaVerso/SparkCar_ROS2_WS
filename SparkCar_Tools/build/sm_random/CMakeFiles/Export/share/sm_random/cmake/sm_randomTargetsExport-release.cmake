#----------------------------------------------------------------
# Generated CMake target import file for configuration "Release".
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Import target "sm_random::sm_random" for configuration "Release"
set_property(TARGET sm_random::sm_random APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(sm_random::sm_random PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_RELEASE "CXX"
  IMPORTED_LOCATION_RELEASE "${_IMPORT_PREFIX}/lib/libsm_random.a"
  )

list(APPEND _IMPORT_CHECK_TARGETS sm_random::sm_random )
list(APPEND _IMPORT_CHECK_FILES_FOR_sm_random::sm_random "${_IMPORT_PREFIX}/lib/libsm_random.a" )

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)
