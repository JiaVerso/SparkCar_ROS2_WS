#----------------------------------------------------------------
# Generated CMake target import file for configuration "Release".
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Import target "sm_logging::sm_logging" for configuration "Release"
set_property(TARGET sm_logging::sm_logging APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(sm_logging::sm_logging PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_RELEASE "CXX"
  IMPORTED_LOCATION_RELEASE "${_IMPORT_PREFIX}/lib/libsm_logging.a"
  )

list(APPEND _IMPORT_CHECK_TARGETS sm_logging::sm_logging )
list(APPEND _IMPORT_CHECK_FILES_FOR_sm_logging::sm_logging "${_IMPORT_PREFIX}/lib/libsm_logging.a" )

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)
