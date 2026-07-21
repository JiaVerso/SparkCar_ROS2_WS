#----------------------------------------------------------------
# Generated CMake target import file for configuration "Release".
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Import target "sm_property_tree::sm_property_tree" for configuration "Release"
set_property(TARGET sm_property_tree::sm_property_tree APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(sm_property_tree::sm_property_tree PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_RELEASE "CXX"
  IMPORTED_LOCATION_RELEASE "${_IMPORT_PREFIX}/lib/libsm_property_tree.a"
  )

list(APPEND _IMPORT_CHECK_TARGETS sm_property_tree::sm_property_tree )
list(APPEND _IMPORT_CHECK_FILES_FOR_sm_property_tree::sm_property_tree "${_IMPORT_PREFIX}/lib/libsm_property_tree.a" )

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)
