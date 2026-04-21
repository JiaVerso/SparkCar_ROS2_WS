// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from interface:srv/SavePoses.idl
// generated code does not contain a copyright notice

#ifndef INTERFACE__SRV__DETAIL__SAVE_POSES__BUILDER_HPP_
#define INTERFACE__SRV__DETAIL__SAVE_POSES__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "interface/srv/detail/save_poses__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace interface
{

namespace srv
{

namespace builder
{

class Init_SavePoses_Request_file_path
{
public:
  Init_SavePoses_Request_file_path()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::interface::srv::SavePoses_Request file_path(::interface::srv::SavePoses_Request::_file_path_type arg)
  {
    msg_.file_path = std::move(arg);
    return std::move(msg_);
  }

private:
  ::interface::srv::SavePoses_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::interface::srv::SavePoses_Request>()
{
  return interface::srv::builder::Init_SavePoses_Request_file_path();
}

}  // namespace interface


namespace interface
{

namespace srv
{

namespace builder
{

class Init_SavePoses_Response_message
{
public:
  explicit Init_SavePoses_Response_message(::interface::srv::SavePoses_Response & msg)
  : msg_(msg)
  {}
  ::interface::srv::SavePoses_Response message(::interface::srv::SavePoses_Response::_message_type arg)
  {
    msg_.message = std::move(arg);
    return std::move(msg_);
  }

private:
  ::interface::srv::SavePoses_Response msg_;
};

class Init_SavePoses_Response_success
{
public:
  Init_SavePoses_Response_success()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_SavePoses_Response_message success(::interface::srv::SavePoses_Response::_success_type arg)
  {
    msg_.success = std::move(arg);
    return Init_SavePoses_Response_message(msg_);
  }

private:
  ::interface::srv::SavePoses_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::interface::srv::SavePoses_Response>()
{
  return interface::srv::builder::Init_SavePoses_Response_success();
}

}  // namespace interface

#endif  // INTERFACE__SRV__DETAIL__SAVE_POSES__BUILDER_HPP_
