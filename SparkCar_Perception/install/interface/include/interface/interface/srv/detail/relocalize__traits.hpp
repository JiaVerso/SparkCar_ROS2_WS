// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from interface:srv/Relocalize.idl
// generated code does not contain a copyright notice

#ifndef INTERFACE__SRV__DETAIL__RELOCALIZE__TRAITS_HPP_
#define INTERFACE__SRV__DETAIL__RELOCALIZE__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "interface/srv/detail/relocalize__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

namespace interface
{

namespace srv
{

inline void to_flow_style_yaml(
  const Relocalize_Request & msg,
  std::ostream & out)
{
  out << "{";
  // member: pcd_path
  {
    out << "pcd_path: ";
    rosidl_generator_traits::value_to_yaml(msg.pcd_path, out);
    out << ", ";
  }

  // member: x
  {
    out << "x: ";
    rosidl_generator_traits::value_to_yaml(msg.x, out);
    out << ", ";
  }

  // member: y
  {
    out << "y: ";
    rosidl_generator_traits::value_to_yaml(msg.y, out);
    out << ", ";
  }

  // member: z
  {
    out << "z: ";
    rosidl_generator_traits::value_to_yaml(msg.z, out);
    out << ", ";
  }

  // member: yaw
  {
    out << "yaw: ";
    rosidl_generator_traits::value_to_yaml(msg.yaw, out);
    out << ", ";
  }

  // member: pitch
  {
    out << "pitch: ";
    rosidl_generator_traits::value_to_yaml(msg.pitch, out);
    out << ", ";
  }

  // member: roll
  {
    out << "roll: ";
    rosidl_generator_traits::value_to_yaml(msg.roll, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const Relocalize_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: pcd_path
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "pcd_path: ";
    rosidl_generator_traits::value_to_yaml(msg.pcd_path, out);
    out << "\n";
  }

  // member: x
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "x: ";
    rosidl_generator_traits::value_to_yaml(msg.x, out);
    out << "\n";
  }

  // member: y
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "y: ";
    rosidl_generator_traits::value_to_yaml(msg.y, out);
    out << "\n";
  }

  // member: z
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "z: ";
    rosidl_generator_traits::value_to_yaml(msg.z, out);
    out << "\n";
  }

  // member: yaw
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "yaw: ";
    rosidl_generator_traits::value_to_yaml(msg.yaw, out);
    out << "\n";
  }

  // member: pitch
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "pitch: ";
    rosidl_generator_traits::value_to_yaml(msg.pitch, out);
    out << "\n";
  }

  // member: roll
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "roll: ";
    rosidl_generator_traits::value_to_yaml(msg.roll, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const Relocalize_Request & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace srv

}  // namespace interface

namespace rosidl_generator_traits
{

[[deprecated("use interface::srv::to_block_style_yaml() instead")]]
inline void to_yaml(
  const interface::srv::Relocalize_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  interface::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use interface::srv::to_yaml() instead")]]
inline std::string to_yaml(const interface::srv::Relocalize_Request & msg)
{
  return interface::srv::to_yaml(msg);
}

template<>
inline const char * data_type<interface::srv::Relocalize_Request>()
{
  return "interface::srv::Relocalize_Request";
}

template<>
inline const char * name<interface::srv::Relocalize_Request>()
{
  return "interface/srv/Relocalize_Request";
}

template<>
struct has_fixed_size<interface::srv::Relocalize_Request>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<interface::srv::Relocalize_Request>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<interface::srv::Relocalize_Request>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace interface
{

namespace srv
{

inline void to_flow_style_yaml(
  const Relocalize_Response & msg,
  std::ostream & out)
{
  out << "{";
  // member: success
  {
    out << "success: ";
    rosidl_generator_traits::value_to_yaml(msg.success, out);
    out << ", ";
  }

  // member: message
  {
    out << "message: ";
    rosidl_generator_traits::value_to_yaml(msg.message, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const Relocalize_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: success
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "success: ";
    rosidl_generator_traits::value_to_yaml(msg.success, out);
    out << "\n";
  }

  // member: message
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "message: ";
    rosidl_generator_traits::value_to_yaml(msg.message, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const Relocalize_Response & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace srv

}  // namespace interface

namespace rosidl_generator_traits
{

[[deprecated("use interface::srv::to_block_style_yaml() instead")]]
inline void to_yaml(
  const interface::srv::Relocalize_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  interface::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use interface::srv::to_yaml() instead")]]
inline std::string to_yaml(const interface::srv::Relocalize_Response & msg)
{
  return interface::srv::to_yaml(msg);
}

template<>
inline const char * data_type<interface::srv::Relocalize_Response>()
{
  return "interface::srv::Relocalize_Response";
}

template<>
inline const char * name<interface::srv::Relocalize_Response>()
{
  return "interface/srv/Relocalize_Response";
}

template<>
struct has_fixed_size<interface::srv::Relocalize_Response>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<interface::srv::Relocalize_Response>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<interface::srv::Relocalize_Response>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<interface::srv::Relocalize>()
{
  return "interface::srv::Relocalize";
}

template<>
inline const char * name<interface::srv::Relocalize>()
{
  return "interface/srv/Relocalize";
}

template<>
struct has_fixed_size<interface::srv::Relocalize>
  : std::integral_constant<
    bool,
    has_fixed_size<interface::srv::Relocalize_Request>::value &&
    has_fixed_size<interface::srv::Relocalize_Response>::value
  >
{
};

template<>
struct has_bounded_size<interface::srv::Relocalize>
  : std::integral_constant<
    bool,
    has_bounded_size<interface::srv::Relocalize_Request>::value &&
    has_bounded_size<interface::srv::Relocalize_Response>::value
  >
{
};

template<>
struct is_service<interface::srv::Relocalize>
  : std::true_type
{
};

template<>
struct is_service_request<interface::srv::Relocalize_Request>
  : std::true_type
{
};

template<>
struct is_service_response<interface::srv::Relocalize_Response>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits

#endif  // INTERFACE__SRV__DETAIL__RELOCALIZE__TRAITS_HPP_
