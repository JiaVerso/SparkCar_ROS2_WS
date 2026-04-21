// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from interface:srv/SaveMaps.idl
// generated code does not contain a copyright notice

#ifndef INTERFACE__SRV__DETAIL__SAVE_MAPS__TRAITS_HPP_
#define INTERFACE__SRV__DETAIL__SAVE_MAPS__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "interface/srv/detail/save_maps__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

namespace interface
{

namespace srv
{

inline void to_flow_style_yaml(
  const SaveMaps_Request & msg,
  std::ostream & out)
{
  out << "{";
  // member: file_path
  {
    out << "file_path: ";
    rosidl_generator_traits::value_to_yaml(msg.file_path, out);
    out << ", ";
  }

  // member: save_patches
  {
    out << "save_patches: ";
    rosidl_generator_traits::value_to_yaml(msg.save_patches, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const SaveMaps_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: file_path
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "file_path: ";
    rosidl_generator_traits::value_to_yaml(msg.file_path, out);
    out << "\n";
  }

  // member: save_patches
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "save_patches: ";
    rosidl_generator_traits::value_to_yaml(msg.save_patches, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const SaveMaps_Request & msg, bool use_flow_style = false)
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
  const interface::srv::SaveMaps_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  interface::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use interface::srv::to_yaml() instead")]]
inline std::string to_yaml(const interface::srv::SaveMaps_Request & msg)
{
  return interface::srv::to_yaml(msg);
}

template<>
inline const char * data_type<interface::srv::SaveMaps_Request>()
{
  return "interface::srv::SaveMaps_Request";
}

template<>
inline const char * name<interface::srv::SaveMaps_Request>()
{
  return "interface/srv/SaveMaps_Request";
}

template<>
struct has_fixed_size<interface::srv::SaveMaps_Request>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<interface::srv::SaveMaps_Request>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<interface::srv::SaveMaps_Request>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace interface
{

namespace srv
{

inline void to_flow_style_yaml(
  const SaveMaps_Response & msg,
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
  const SaveMaps_Response & msg,
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

inline std::string to_yaml(const SaveMaps_Response & msg, bool use_flow_style = false)
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
  const interface::srv::SaveMaps_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  interface::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use interface::srv::to_yaml() instead")]]
inline std::string to_yaml(const interface::srv::SaveMaps_Response & msg)
{
  return interface::srv::to_yaml(msg);
}

template<>
inline const char * data_type<interface::srv::SaveMaps_Response>()
{
  return "interface::srv::SaveMaps_Response";
}

template<>
inline const char * name<interface::srv::SaveMaps_Response>()
{
  return "interface/srv/SaveMaps_Response";
}

template<>
struct has_fixed_size<interface::srv::SaveMaps_Response>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<interface::srv::SaveMaps_Response>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<interface::srv::SaveMaps_Response>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<interface::srv::SaveMaps>()
{
  return "interface::srv::SaveMaps";
}

template<>
inline const char * name<interface::srv::SaveMaps>()
{
  return "interface/srv/SaveMaps";
}

template<>
struct has_fixed_size<interface::srv::SaveMaps>
  : std::integral_constant<
    bool,
    has_fixed_size<interface::srv::SaveMaps_Request>::value &&
    has_fixed_size<interface::srv::SaveMaps_Response>::value
  >
{
};

template<>
struct has_bounded_size<interface::srv::SaveMaps>
  : std::integral_constant<
    bool,
    has_bounded_size<interface::srv::SaveMaps_Request>::value &&
    has_bounded_size<interface::srv::SaveMaps_Response>::value
  >
{
};

template<>
struct is_service<interface::srv::SaveMaps>
  : std::true_type
{
};

template<>
struct is_service_request<interface::srv::SaveMaps_Request>
  : std::true_type
{
};

template<>
struct is_service_response<interface::srv::SaveMaps_Response>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits

#endif  // INTERFACE__SRV__DETAIL__SAVE_MAPS__TRAITS_HPP_
