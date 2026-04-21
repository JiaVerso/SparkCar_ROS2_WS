// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from interface:srv/SavePoses.idl
// generated code does not contain a copyright notice

#ifndef INTERFACE__SRV__DETAIL__SAVE_POSES__STRUCT_H_
#define INTERFACE__SRV__DETAIL__SAVE_POSES__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

// Include directives for member types
// Member 'file_path'
#include "rosidl_runtime_c/string.h"

/// Struct defined in srv/SavePoses in the package interface.
typedef struct interface__srv__SavePoses_Request
{
  rosidl_runtime_c__String file_path;
} interface__srv__SavePoses_Request;

// Struct for a sequence of interface__srv__SavePoses_Request.
typedef struct interface__srv__SavePoses_Request__Sequence
{
  interface__srv__SavePoses_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} interface__srv__SavePoses_Request__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'message'
// already included above
// #include "rosidl_runtime_c/string.h"

/// Struct defined in srv/SavePoses in the package interface.
typedef struct interface__srv__SavePoses_Response
{
  bool success;
  rosidl_runtime_c__String message;
} interface__srv__SavePoses_Response;

// Struct for a sequence of interface__srv__SavePoses_Response.
typedef struct interface__srv__SavePoses_Response__Sequence
{
  interface__srv__SavePoses_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} interface__srv__SavePoses_Response__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // INTERFACE__SRV__DETAIL__SAVE_POSES__STRUCT_H_
