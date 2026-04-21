// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from interface:srv/IsValid.idl
// generated code does not contain a copyright notice

#ifndef INTERFACE__SRV__DETAIL__IS_VALID__TRAITS_HPP_
#define INTERFACE__SRV__DETAIL__IS_VALID__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "interface/srv/detail/is_valid__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

namespace interface
{

namespace srv
{

inline void to_flow_style_yaml(
  const IsValid_Request & msg,
  std::ostream & out)
{
  out << "{";
  // member: code
  {
    out << "code: ";
    rosidl_generator_traits::value_to_yaml(msg.code, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const IsValid_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: code
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "code: ";
    rosidl_generator_traits::value_to_yaml(msg.code, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const IsValid_Request & msg, bool use_flow_style = false)
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
  const interface::srv::IsValid_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  interface::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use interface::srv::to_yaml() instead")]]
inline std::string to_yaml(const interface::srv::IsValid_Request & msg)
{
  return interface::srv::to_yaml(msg);
}

template<>
inline const char * data_type<interface::srv::IsValid_Request>()
{
  return "interface::srv::IsValid_Request";
}

template<>
inline const char * name<interface::srv::IsValid_Request>()
{
  return "interface/srv/IsValid_Request";
}

template<>
struct has_fixed_size<interface::srv::IsValid_Request>
  : std::integral_constant<bool, true> {};

template<>
struct has_bounded_size<interface::srv::IsValid_Request>
  : std::integral_constant<bool, true> {};

template<>
struct is_message<interface::srv::IsValid_Request>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace interface
{

namespace srv
{

inline void to_flow_style_yaml(
  const IsValid_Response & msg,
  std::ostream & out)
{
  out << "{";
  // member: valid
  {
    out << "valid: ";
    rosidl_generator_traits::value_to_yaml(msg.valid, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const IsValid_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: valid
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "valid: ";
    rosidl_generator_traits::value_to_yaml(msg.valid, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const IsValid_Response & msg, bool use_flow_style = false)
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
  const interface::srv::IsValid_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  interface::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use interface::srv::to_yaml() instead")]]
inline std::string to_yaml(const interface::srv::IsValid_Response & msg)
{
  return interface::srv::to_yaml(msg);
}

template<>
inline const char * data_type<interface::srv::IsValid_Response>()
{
  return "interface::srv::IsValid_Response";
}

template<>
inline const char * name<interface::srv::IsValid_Response>()
{
  return "interface/srv/IsValid_Response";
}

template<>
struct has_fixed_size<interface::srv::IsValid_Response>
  : std::integral_constant<bool, true> {};

template<>
struct has_bounded_size<interface::srv::IsValid_Response>
  : std::integral_constant<bool, true> {};

template<>
struct is_message<interface::srv::IsValid_Response>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<interface::srv::IsValid>()
{
  return "interface::srv::IsValid";
}

template<>
inline const char * name<interface::srv::IsValid>()
{
  return "interface/srv/IsValid";
}

template<>
struct has_fixed_size<interface::srv::IsValid>
  : std::integral_constant<
    bool,
    has_fixed_size<interface::srv::IsValid_Request>::value &&
    has_fixed_size<interface::srv::IsValid_Response>::value
  >
{
};

template<>
struct has_bounded_size<interface::srv::IsValid>
  : std::integral_constant<
    bool,
    has_bounded_size<interface::srv::IsValid_Request>::value &&
    has_bounded_size<interface::srv::IsValid_Response>::value
  >
{
};

template<>
struct is_service<interface::srv::IsValid>
  : std::true_type
{
};

template<>
struct is_service_request<interface::srv::IsValid_Request>
  : std::true_type
{
};

template<>
struct is_service_response<interface::srv::IsValid_Response>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits

#endif  // INTERFACE__SRV__DETAIL__IS_VALID__TRAITS_HPP_
