// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from interface:srv/IsValid.idl
// generated code does not contain a copyright notice

#ifndef INTERFACE__SRV__DETAIL__IS_VALID__BUILDER_HPP_
#define INTERFACE__SRV__DETAIL__IS_VALID__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "interface/srv/detail/is_valid__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace interface
{

namespace srv
{

namespace builder
{

class Init_IsValid_Request_code
{
public:
  Init_IsValid_Request_code()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::interface::srv::IsValid_Request code(::interface::srv::IsValid_Request::_code_type arg)
  {
    msg_.code = std::move(arg);
    return std::move(msg_);
  }

private:
  ::interface::srv::IsValid_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::interface::srv::IsValid_Request>()
{
  return interface::srv::builder::Init_IsValid_Request_code();
}

}  // namespace interface


namespace interface
{

namespace srv
{

namespace builder
{

class Init_IsValid_Response_valid
{
public:
  Init_IsValid_Response_valid()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::interface::srv::IsValid_Response valid(::interface::srv::IsValid_Response::_valid_type arg)
  {
    msg_.valid = std::move(arg);
    return std::move(msg_);
  }

private:
  ::interface::srv::IsValid_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::interface::srv::IsValid_Response>()
{
  return interface::srv::builder::Init_IsValid_Response_valid();
}

}  // namespace interface

#endif  // INTERFACE__SRV__DETAIL__IS_VALID__BUILDER_HPP_
