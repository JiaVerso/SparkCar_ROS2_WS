#----------------------------------------------------------------
# Generated CMake target import file for configuration "Release".
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Import target "sparse_block_matrix::sparse_block_matrix" for configuration "Release"
set_property(TARGET sparse_block_matrix::sparse_block_matrix APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(sparse_block_matrix::sparse_block_matrix PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_RELEASE "CXX"
  IMPORTED_LOCATION_RELEASE "${_IMPORT_PREFIX}/lib/libsparse_block_matrix.a"
  )

list(APPEND _IMPORT_CHECK_TARGETS sparse_block_matrix::sparse_block_matrix )
list(APPEND _IMPORT_CHECK_FILES_FOR_sparse_block_matrix::sparse_block_matrix "${_IMPORT_PREFIX}/lib/libsparse_block_matrix.a" )

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)
