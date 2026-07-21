#----------------------------------------------------------------
# Generated CMake target import file for configuration "Release".
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Import target "sm_kinematics::sm_kinematics" for configuration "Release"
set_property(TARGET sm_kinematics::sm_kinematics APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(sm_kinematics::sm_kinematics PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_RELEASE "CXX"
  IMPORTED_LOCATION_RELEASE "${_IMPORT_PREFIX}/lib/libsm_kinematics.a"
  )

list(APPEND _IMPORT_CHECK_TARGETS sm_kinematics::sm_kinematics )
list(APPEND _IMPORT_CHECK_FILES_FOR_sm_kinematics::sm_kinematics "${_IMPORT_PREFIX}/lib/libsm_kinematics.a" )

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)
