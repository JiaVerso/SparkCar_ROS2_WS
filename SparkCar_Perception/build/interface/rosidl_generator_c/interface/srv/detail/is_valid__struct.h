// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from interface:srv/IsValid.idl
// generated code does not contain a copyright notice

#ifndef INTERFACE__SRV__DETAIL__IS_VALID__STRUCT_H_
#define INTERFACE__SRV__DETAIL__IS_VALID__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Struct defined in srv/IsValid in the package interface.
typedef struct interface__srv__IsValid_Request
{
  int32_t code;
} interface__srv__IsValid_Request;

// Struct for a sequence of interface__srv__IsValid_Request.
typedef struct interface__srv__IsValid_Request__Sequence
{
  interface__srv__IsValid_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} interface__srv__IsValid_Request__Sequence;


// Constants defined in the message

/// Struct defined in srv/IsValid in the package interface.
typedef struct interface__srv__IsValid_Response
{
  bool valid;
} interface__srv__IsValid_Response;

// Struct for a sequence of interface__srv__IsValid_Response.
typedef struct interface__srv__IsValid_Response__Sequence
{
  interface__srv__IsValid_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} interface__srv__IsValid_Response__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // INTERFACE__SRV__DETAIL__IS_VALID__STRUCT_H_
