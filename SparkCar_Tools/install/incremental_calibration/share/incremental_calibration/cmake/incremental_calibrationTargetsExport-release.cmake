#----------------------------------------------------------------
# Generated CMake target import file for configuration "Release".
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Import target "incremental_calibration::incremental_calibration" for configuration "Release"
set_property(TARGET incremental_calibration::incremental_calibration APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(incremental_calibration::incremental_calibration PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_RELEASE "CXX"
  IMPORTED_LOCATION_RELEASE "${_IMPORT_PREFIX}/lib/libincremental_calibration.a"
  )

list(APPEND _IMPORT_CHECK_TARGETS incremental_calibration::incremental_calibration )
list(APPEND _IMPORT_CHECK_FILES_FOR_incremental_calibration::incremental_calibration "${_IMPORT_PREFIX}/lib/libincremental_calibration.a" )

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)
