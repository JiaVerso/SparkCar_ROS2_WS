#----------------------------------------------------------------
# Generated CMake target import file for configuration "Release".
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Import target "sm_matrix_archive::sm_matrix_archive" for configuration "Release"
set_property(TARGET sm_matrix_archive::sm_matrix_archive APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(sm_matrix_archive::sm_matrix_archive PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_RELEASE "CXX"
  IMPORTED_LOCATION_RELEASE "${_IMPORT_PREFIX}/lib/libsm_matrix_archive.a"
  )

list(APPEND _IMPORT_CHECK_TARGETS sm_matrix_archive::sm_matrix_archive )
list(APPEND _IMPORT_CHECK_FILES_FOR_sm_matrix_archive::sm_matrix_archive "${_IMPORT_PREFIX}/lib/libsm_matrix_archive.a" )

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)
