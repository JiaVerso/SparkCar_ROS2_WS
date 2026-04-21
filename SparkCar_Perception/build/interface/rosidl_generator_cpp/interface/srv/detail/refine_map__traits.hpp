// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from interface:srv/RefineMap.idl
// generated code does not contain a copyright notice

#ifndef INTERFACE__SRV__DETAIL__REFINE_MAP__TRAITS_HPP_
#define INTERFACE__SRV__DETAIL__REFINE_MAP__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "interface/srv/detail/refine_map__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

namespace interface
{

namespace srv
{

inline void to_flow_style_yaml(
  const RefineMap_Request & msg,
  std::ostream & out)
{
  out << "{";
  // member: maps_path
  {
    out << "maps_path: ";
    rosidl_generator_traits::value_to_yaml(msg.maps_path, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const RefineMap_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: maps_path
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "maps_path: ";
    rosidl_generator_traits::value_to_yaml(msg.maps_path, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const RefineMap_Request & msg, bool use_flow_style = false)
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
  const interface::srv::RefineMap_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  interface::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use interface::srv::to_yaml() instead")]]
inline std::string to_yaml(const interface::srv::RefineMap_Request & msg)
{
  return interface::srv::to_yaml(msg);
}

template<>
inline const char * data_type<interface::srv::RefineMap_Request>()
{
  return "interface::srv::RefineMap_Request";
}

template<>
inline const char * name<interface::srv::RefineMap_Request>()
{
  return "interface/srv/RefineMap_Request";
}

template<>
struct has_fixed_size<interface::srv::RefineMap_Request>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<interface::srv::RefineMap_Request>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<interface::srv::RefineMap_Request>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace interface
{

namespace srv
{

inline void to_flow_style_yaml(
  const RefineMap_Response & msg,
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
  const RefineMap_Response & msg,
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

inline std::string to_yaml(const RefineMap_Response & msg, bool use_flow_style = false)
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
  const interface::srv::RefineMap_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  interface::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use interface::srv::to_yaml() instead")]]
inline std::string to_yaml(const interface::srv::RefineMap_Response & msg)
{
  return interface::srv::to_yaml(msg);
}

template<>
inline const char * data_type<interface::srv::RefineMap_Response>()
{
  return "interface::srv::RefineMap_Response";
}

template<>
inline const char * name<interface::srv::RefineMap_Response>()
{
  return "interface/srv/RefineMap_Response";
}

template<>
struct has_fixed_size<interface::srv::RefineMap_Response>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<interface::srv::RefineMap_Response>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<interface::srv::RefineMap_Response>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<interface::srv::RefineMap>()
{
  return "interface::srv::RefineMap";
}

template<>
inline const char * name<interface::srv::RefineMap>()
{
  return "interface/srv/RefineMap";
}

template<>
struct has_fixed_size<interface::srv::RefineMap>
  : std::integral_constant<
    bool,
    has_fixed_size<interface::srv::RefineMap_Request>::value &&
    has_fixed_size<interface::srv::RefineMap_Response>::value
  >
{
};

template<>
struct has_bounded_size<interface::srv::RefineMap>
  : std::integral_constant<
    bool,
    has_bounded_size<interface::srv::RefineMap_Request>::value &&
    has_bounded_size<interface::srv::RefineMap_Response>::value
  >
{
};

template<>
struct is_service<interface::srv::RefineMap>
  : std::true_type
{
};

template<>
struct is_service_request<interface::srv::RefineMap_Request>
  : std::true_type
{
};

template<>
struct is_service_response<interface::srv::RefineMap_Response>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits

#endif  // INTERFACE__SRV__DETAIL__REFINE_MAP__TRAITS_HPP_
