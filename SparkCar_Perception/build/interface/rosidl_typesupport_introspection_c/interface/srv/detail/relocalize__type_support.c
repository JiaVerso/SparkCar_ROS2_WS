// generated from rosidl_typesupport_introspection_c/resource/idl__type_support.c.em
// with input from interface:srv/Relocalize.idl
// generated code does not contain a copyright notice

#include <stddef.h>
#include "interface/srv/detail/relocalize__rosidl_typesupport_introspection_c.h"
#include "interface/msg/rosidl_typesupport_introspection_c__visibility_control.h"
#include "rosidl_typesupport_introspection_c/field_types.h"
#include "rosidl_typesupport_introspection_c/identifier.h"
#include "rosidl_typesupport_introspection_c/message_introspection.h"
#include "interface/srv/detail/relocalize__functions.h"
#include "interface/srv/detail/relocalize__struct.h"


// Include directives for member types
// Member `pcd_path`
#include "rosidl_runtime_c/string_functions.h"

#ifdef __cplusplus
extern "C"
{
#endif

void interface__srv__Relocalize_Request__rosidl_typesupport_introspection_c__Relocalize_Request_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  interface__srv__Relocalize_Request__init(message_memory);
}

void interface__srv__Relocalize_Request__rosidl_typesupport_introspection_c__Relocalize_Request_fini_function(void * message_memory)
{
  interface__srv__Relocalize_Request__fini(message_memory);
}

static rosidl_typesupport_introspection_c__MessageMember interface__srv__Relocalize_Request__rosidl_typesupport_introspection_c__Relocalize_Request_message_member_array[7] = {
  {
    "pcd_path",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_STRING,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(interface__srv__Relocalize_Request, pcd_path),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "x",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_FLOAT,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(interface__srv__Relocalize_Request, x),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "y",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_FLOAT,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(interface__srv__Relocalize_Request, y),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "z",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_FLOAT,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(interface__srv__Relocalize_Request, z),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "yaw",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_FLOAT,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(interface__srv__Relocalize_Request, yaw),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "pitch",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_FLOAT,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(interface__srv__Relocalize_Request, pitch),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "roll",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_FLOAT,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(interface__srv__Relocalize_Request, roll),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers interface__srv__Relocalize_Request__rosidl_typesupport_introspection_c__Relocalize_Request_message_members = {
  "interface__srv",  // message namespace
  "Relocalize_Request",  // message name
  7,  // number of fields
  sizeof(interface__srv__Relocalize_Request),
  interface__srv__Relocalize_Request__rosidl_typesupport_introspection_c__Relocalize_Request_message_member_array,  // message members
  interface__srv__Relocalize_Request__rosidl_typesupport_introspection_c__Relocalize_Request_init_function,  // function to initialize message memory (memory has to be allocated)
  interface__srv__Relocalize_Request__rosidl_typesupport_introspection_c__Relocalize_Request_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t interface__srv__Relocalize_Request__rosidl_typesupport_introspection_c__Relocalize_Request_message_type_support_handle = {
  0,
  &interface__srv__Relocalize_Request__rosidl_typesupport_introspection_c__Relocalize_Request_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_interface
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, interface, srv, Relocalize_Request)() {
  if (!interface__srv__Relocalize_Request__rosidl_typesupport_introspection_c__Relocalize_Request_message_type_support_handle.typesupport_identifier) {
    interface__srv__Relocalize_Request__rosidl_typesupport_introspection_c__Relocalize_Request_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &interface__srv__Relocalize_Request__rosidl_typesupport_introspection_c__Relocalize_Request_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif

// already included above
// #include <stddef.h>
// already included above
// #include "interface/srv/detail/relocalize__rosidl_typesupport_introspection_c.h"
// already included above
// #include "interface/msg/rosidl_typesupport_introspection_c__visibility_control.h"
// already included above
// #include "rosidl_typesupport_introspection_c/field_types.h"
// already included above
// #include "rosidl_typesupport_introspection_c/identifier.h"
// already included above
// #include "rosidl_typesupport_introspection_c/message_introspection.h"
// already included above
// #include "interface/srv/detail/relocalize__functions.h"
// already included above
// #include "interface/srv/detail/relocalize__struct.h"


// Include directives for member types
// Member `message`
// already included above
// #include "rosidl_runtime_c/string_functions.h"

#ifdef __cplusplus
extern "C"
{
#endif

void interface__srv__Relocalize_Response__rosidl_typesupport_introspection_c__Relocalize_Response_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  interface__srv__Relocalize_Response__init(message_memory);
}

void interface__srv__Relocalize_Response__rosidl_typesupport_introspection_c__Relocalize_Response_fini_function(void * message_memory)
{
  interface__srv__Relocalize_Response__fini(message_memory);
}

static rosidl_typesupport_introspection_c__MessageMember interface__srv__Relocalize_Response__rosidl_typesupport_introspection_c__Relocalize_Response_message_member_array[2] = {
  {
    "success",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_BOOLEAN,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(interface__srv__Relocalize_Response, success),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "message",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_STRING,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(interface__srv__Relocalize_Response, message),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers interface__srv__Relocalize_Response__rosidl_typesupport_introspection_c__Relocalize_Response_message_members = {
  "interface__srv",  // message namespace
  "Relocalize_Response",  // message name
  2,  // number of fields
  sizeof(interface__srv__Relocalize_Response),
  interface__srv__Relocalize_Response__rosidl_typesupport_introspection_c__Relocalize_Response_message_member_array,  // message members
  interface__srv__Relocalize_Response__rosidl_typesupport_introspection_c__Relocalize_Response_init_function,  // function to initialize message memory (memory has to be allocated)
  interface__srv__Relocalize_Response__rosidl_typesupport_introspection_c__Relocalize_Response_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t interface__srv__Relocalize_Response__rosidl_typesupport_introspection_c__Relocalize_Response_message_type_support_handle = {
  0,
  &interface__srv__Relocalize_Response__rosidl_typesupport_introspection_c__Relocalize_Response_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_interface
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, interface, srv, Relocalize_Response)() {
  if (!interface__srv__Relocalize_Response__rosidl_typesupport_introspection_c__Relocalize_Response_message_type_support_handle.typesupport_identifier) {
    interface__srv__Relocalize_Response__rosidl_typesupport_introspection_c__Relocalize_Response_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &interface__srv__Relocalize_Response__rosidl_typesupport_introspection_c__Relocalize_Response_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif

#include "rosidl_runtime_c/service_type_support_struct.h"
// already included above
// #include "interface/msg/rosidl_typesupport_introspection_c__visibility_control.h"
// already included above
// #include "interface/srv/detail/relocalize__rosidl_typesupport_introspection_c.h"
// already included above
// #include "rosidl_typesupport_introspection_c/identifier.h"
#include "rosidl_typesupport_introspection_c/service_introspection.h"

// this is intentionally not const to allow initialization later to prevent an initialization race
static rosidl_typesupport_introspection_c__ServiceMembers interface__srv__detail__relocalize__rosidl_typesupport_introspection_c__Relocalize_service_members = {
  "interface__srv",  // service namespace
  "Relocalize",  // service name
  // these two fields are initialized below on the first access
  NULL,  // request message
  // interface__srv__detail__relocalize__rosidl_typesupport_introspection_c__Relocalize_Request_message_type_support_handle,
  NULL  // response message
  // interface__srv__detail__relocalize__rosidl_typesupport_introspection_c__Relocalize_Response_message_type_support_handle
};

static rosidl_service_type_support_t interface__srv__detail__relocalize__rosidl_typesupport_introspection_c__Relocalize_service_type_support_handle = {
  0,
  &interface__srv__detail__relocalize__rosidl_typesupport_introspection_c__Relocalize_service_members,
  get_service_typesupport_handle_function,
};

// Forward declaration of request/response type support functions
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, interface, srv, Relocalize_Request)();

const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, interface, srv, Relocalize_Response)();

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_interface
const rosidl_service_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_introspection_c, interface, srv, Relocalize)() {
  if (!interface__srv__detail__relocalize__rosidl_typesupport_introspection_c__Relocalize_service_type_support_handle.typesupport_identifier) {
    interface__srv__detail__relocalize__rosidl_typesupport_introspection_c__Relocalize_service_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  rosidl_typesupport_introspection_c__ServiceMembers * service_members =
    (rosidl_typesupport_introspection_c__ServiceMembers *)interface__srv__detail__relocalize__rosidl_typesupport_introspection_c__Relocalize_service_type_support_handle.data;

  if (!service_members->request_members_) {
    service_members->request_members_ =
      (const rosidl_typesupport_introspection_c__MessageMembers *)
      ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, interface, srv, Relocalize_Request)()->data;
  }
  if (!service_members->response_members_) {
    service_members->response_members_ =
      (const rosidl_typesupport_introspection_c__MessageMembers *)
      ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, interface, srv, Relocalize_Response)()->data;
  }

  return &interface__srv__detail__relocalize__rosidl_typesupport_introspection_c__Relocalize_service_type_support_handle;
}
