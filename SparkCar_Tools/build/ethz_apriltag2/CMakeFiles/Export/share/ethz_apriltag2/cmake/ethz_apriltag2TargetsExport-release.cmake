#----------------------------------------------------------------
# Generated CMake target import file for configuration "Release".
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Import target "ethz_apriltag2::ethz_apriltag2" for configuration "Release"
set_property(TARGET ethz_apriltag2::ethz_apriltag2 APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(ethz_apriltag2::ethz_apriltag2 PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_RELEASE "CXX"
  IMPORTED_LOCATION_RELEASE "${_IMPORT_PREFIX}/lib/libethz_apriltag2.a"
  )

list(APPEND _IMPORT_CHECK_TARGETS ethz_apriltag2::ethz_apriltag2 )
list(APPEND _IMPORT_CHECK_FILES_FOR_ethz_apriltag2::ethz_apriltag2 "${_IMPORT_PREFIX}/lib/libethz_apriltag2.a" )

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)
