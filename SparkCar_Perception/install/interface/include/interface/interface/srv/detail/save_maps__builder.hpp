// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from interface:srv/SaveMaps.idl
// generated code does not contain a copyright notice

#ifndef INTERFACE__SRV__DETAIL__SAVE_MAPS__BUILDER_HPP_
#define INTERFACE__SRV__DETAIL__SAVE_MAPS__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "interface/srv/detail/save_maps__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace interface
{

namespace srv
{

namespace builder
{

class Init_SaveMaps_Request_save_patches
{
public:
  explicit Init_SaveMaps_Request_save_patches(::interface::srv::SaveMaps_Request & msg)
  : msg_(msg)
  {}
  ::interface::srv::SaveMaps_Request save_patches(::interface::srv::SaveMaps_Request::_save_patches_type arg)
  {
    msg_.save_patches = std::move(arg);
    return std::move(msg_);
  }

private:
  ::interface::srv::SaveMaps_Request msg_;
};

class Init_SaveMaps_Request_file_path
{
public:
  Init_SaveMaps_Request_file_path()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_SaveMaps_Request_save_patches file_path(::interface::srv::SaveMaps_Request::_file_path_type arg)
  {
    msg_.file_path = std::move(arg);
    return Init_SaveMaps_Request_save_patches(msg_);
  }

private:
  ::interface::srv::SaveMaps_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::interface::srv::SaveMaps_Request>()
{
  return interface::srv::builder::Init_SaveMaps_Request_file_path();
}

}  // namespace interface


namespace interface
{

namespace srv
{

namespace builder
{

class Init_SaveMaps_Response_message
{
public:
  explicit Init_SaveMaps_Response_message(::interface::srv::SaveMaps_Response & msg)
  : msg_(msg)
  {}
  ::interface::srv::SaveMaps_Response message(::interface::srv::SaveMaps_Response::_message_type arg)
  {
    msg_.message = std::move(arg);
    return std::move(msg_);
  }

private:
  ::interface::srv::SaveMaps_Response msg_;
};

class Init_SaveMaps_Response_success
{
public:
  Init_SaveMaps_Response_success()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_SaveMaps_Response_message success(::interface::srv::SaveMaps_Response::_success_type arg)
  {
    msg_.success = std::move(arg);
    return Init_SaveMaps_Response_message(msg_);
  }

private:
  ::interface::srv::SaveMaps_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::interface::srv::SaveMaps_Response>()
{
  return interface::srv::builder::Init_SaveMaps_Response_success();
}

}  // namespace interface

#endif  // INTERFACE__SRV__DETAIL__SAVE_MAPS__BUILDER_HPP_
