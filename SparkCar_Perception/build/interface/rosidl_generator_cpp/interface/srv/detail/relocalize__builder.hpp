// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from interface:srv/Relocalize.idl
// generated code does not contain a copyright notice

#ifndef INTERFACE__SRV__DETAIL__RELOCALIZE__BUILDER_HPP_
#define INTERFACE__SRV__DETAIL__RELOCALIZE__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "interface/srv/detail/relocalize__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace interface
{

namespace srv
{

namespace builder
{

class Init_Relocalize_Request_roll
{
public:
  explicit Init_Relocalize_Request_roll(::interface::srv::Relocalize_Request & msg)
  : msg_(msg)
  {}
  ::interface::srv::Relocalize_Request roll(::interface::srv::Relocalize_Request::_roll_type arg)
  {
    msg_.roll = std::move(arg);
    return std::move(msg_);
  }

private:
  ::interface::srv::Relocalize_Request msg_;
};

class Init_Relocalize_Request_pitch
{
public:
  explicit Init_Relocalize_Request_pitch(::interface::srv::Relocalize_Request & msg)
  : msg_(msg)
  {}
  Init_Relocalize_Request_roll pitch(::interface::srv::Relocalize_Request::_pitch_type arg)
  {
    msg_.pitch = std::move(arg);
    return Init_Relocalize_Request_roll(msg_);
  }

private:
  ::interface::srv::Relocalize_Request msg_;
};

class Init_Relocalize_Request_yaw
{
public:
  explicit Init_Relocalize_Request_yaw(::interface::srv::Relocalize_Request & msg)
  : msg_(msg)
  {}
  Init_Relocalize_Request_pitch yaw(::interface::srv::Relocalize_Request::_yaw_type arg)
  {
    msg_.yaw = std::move(arg);
    return Init_Relocalize_Request_pitch(msg_);
  }

private:
  ::interface::srv::Relocalize_Request msg_;
};

class Init_Relocalize_Request_z
{
public:
  explicit Init_Relocalize_Request_z(::interface::srv::Relocalize_Request & msg)
  : msg_(msg)
  {}
  Init_Relocalize_Request_yaw z(::interface::srv::Relocalize_Request::_z_type arg)
  {
    msg_.z = std::move(arg);
    return Init_Relocalize_Request_yaw(msg_);
  }

private:
  ::interface::srv::Relocalize_Request msg_;
};

class Init_Relocalize_Request_y
{
public:
  explicit Init_Relocalize_Request_y(::interface::srv::Relocalize_Request & msg)
  : msg_(msg)
  {}
  Init_Relocalize_Request_z y(::interface::srv::Relocalize_Request::_y_type arg)
  {
    msg_.y = std::move(arg);
    return Init_Relocalize_Request_z(msg_);
  }

private:
  ::interface::srv::Relocalize_Request msg_;
};

class Init_Relocalize_Request_x
{
public:
  explicit Init_Relocalize_Request_x(::interface::srv::Relocalize_Request & msg)
  : msg_(msg)
  {}
  Init_Relocalize_Request_y x(::interface::srv::Relocalize_Request::_x_type arg)
  {
    msg_.x = std::move(arg);
    return Init_Relocalize_Request_y(msg_);
  }

private:
  ::interface::srv::Relocalize_Request msg_;
};

class Init_Relocalize_Request_pcd_path
{
public:
  Init_Relocalize_Request_pcd_path()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_Relocalize_Request_x pcd_path(::interface::srv::Relocalize_Request::_pcd_path_type arg)
  {
    msg_.pcd_path = std::move(arg);
    return Init_Relocalize_Request_x(msg_);
  }

private:
  ::interface::srv::Relocalize_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::interface::srv::Relocalize_Request>()
{
  return interface::srv::builder::Init_Relocalize_Request_pcd_path();
}

}  // namespace interface


namespace interface
{

namespace srv
{

namespace builder
{

class Init_Relocalize_Response_message
{
public:
  explicit Init_Relocalize_Response_message(::interface::srv::Relocalize_Response & msg)
  : msg_(msg)
  {}
  ::interface::srv::Relocalize_Response message(::interface::srv::Relocalize_Response::_message_type arg)
  {
    msg_.message = std::move(arg);
    return std::move(msg_);
  }

private:
  ::interface::srv::Relocalize_Response msg_;
};

class Init_Relocalize_Response_success
{
public:
  Init_Relocalize_Response_success()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_Relocalize_Response_message success(::interface::srv::Relocalize_Response::_success_type arg)
  {
    msg_.success = std::move(arg);
    return Init_Relocalize_Response_message(msg_);
  }

private:
  ::interface::srv::Relocalize_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::interface::srv::Relocalize_Response>()
{
  return interface::srv::builder::Init_Relocalize_Response_success();
}

}  // namespace interface

#endif  // INTERFACE__SRV__DETAIL__RELOCALIZE__BUILDER_HPP_
