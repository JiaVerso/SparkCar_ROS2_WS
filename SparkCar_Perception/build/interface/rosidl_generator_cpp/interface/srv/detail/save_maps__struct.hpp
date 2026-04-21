// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from interface:srv/SaveMaps.idl
// generated code does not contain a copyright notice

#ifndef INTERFACE__SRV__DETAIL__SAVE_MAPS__STRUCT_HPP_
#define INTERFACE__SRV__DETAIL__SAVE_MAPS__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__interface__srv__SaveMaps_Request __attribute__((deprecated))
#else
# define DEPRECATED__interface__srv__SaveMaps_Request __declspec(deprecated)
#endif

namespace interface
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct SaveMaps_Request_
{
  using Type = SaveMaps_Request_<ContainerAllocator>;

  explicit SaveMaps_Request_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->file_path = "";
      this->save_patches = false;
    }
  }

  explicit SaveMaps_Request_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : file_path(_alloc)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->file_path = "";
      this->save_patches = false;
    }
  }

  // field types and members
  using _file_path_type =
    std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>;
  _file_path_type file_path;
  using _save_patches_type =
    bool;
  _save_patches_type save_patches;

  // setters for named parameter idiom
  Type & set__file_path(
    const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> & _arg)
  {
    this->file_path = _arg;
    return *this;
  }
  Type & set__save_patches(
    const bool & _arg)
  {
    this->save_patches = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    interface::srv::SaveMaps_Request_<ContainerAllocator> *;
  using ConstRawPtr =
    const interface::srv::SaveMaps_Request_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<interface::srv::SaveMaps_Request_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<interface::srv::SaveMaps_Request_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      interface::srv::SaveMaps_Request_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<interface::srv::SaveMaps_Request_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      interface::srv::SaveMaps_Request_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<interface::srv::SaveMaps_Request_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<interface::srv::SaveMaps_Request_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<interface::srv::SaveMaps_Request_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__interface__srv__SaveMaps_Request
    std::shared_ptr<interface::srv::SaveMaps_Request_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__interface__srv__SaveMaps_Request
    std::shared_ptr<interface::srv::SaveMaps_Request_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const SaveMaps_Request_ & other) const
  {
    if (this->file_path != other.file_path) {
      return false;
    }
    if (this->save_patches != other.save_patches) {
      return false;
    }
    return true;
  }
  bool operator!=(const SaveMaps_Request_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct SaveMaps_Request_

// alias to use template instance with default allocator
using SaveMaps_Request =
  interface::srv::SaveMaps_Request_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace interface


#ifndef _WIN32
# define DEPRECATED__interface__srv__SaveMaps_Response __attribute__((deprecated))
#else
# define DEPRECATED__interface__srv__SaveMaps_Response __declspec(deprecated)
#endif

namespace interface
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct SaveMaps_Response_
{
  using Type = SaveMaps_Response_<ContainerAllocator>;

  explicit SaveMaps_Response_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->success = false;
      this->message = "";
    }
  }

  explicit SaveMaps_Response_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : message(_alloc)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->success = false;
      this->message = "";
    }
  }

  // field types and members
  using _success_type =
    bool;
  _success_type success;
  using _message_type =
    std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>;
  _message_type message;

  // setters for named parameter idiom
  Type & set__success(
    const bool & _arg)
  {
    this->success = _arg;
    return *this;
  }
  Type & set__message(
    const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> & _arg)
  {
    this->message = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    interface::srv::SaveMaps_Response_<ContainerAllocator> *;
  using ConstRawPtr =
    const interface::srv::SaveMaps_Response_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<interface::srv::SaveMaps_Response_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<interface::srv::SaveMaps_Response_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      interface::srv::SaveMaps_Response_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<interface::srv::SaveMaps_Response_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      interface::srv::SaveMaps_Response_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<interface::srv::SaveMaps_Response_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<interface::srv::SaveMaps_Response_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<interface::srv::SaveMaps_Response_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__interface__srv__SaveMaps_Response
    std::shared_ptr<interface::srv::SaveMaps_Response_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__interface__srv__SaveMaps_Response
    std::shared_ptr<interface::srv::SaveMaps_Response_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const SaveMaps_Response_ & other) const
  {
    if (this->success != other.success) {
      return false;
    }
    if (this->message != other.message) {
      return false;
    }
    return true;
  }
  bool operator!=(const SaveMaps_Response_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct SaveMaps_Response_

// alias to use template instance with default allocator
using SaveMaps_Response =
  interface::srv::SaveMaps_Response_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace interface

namespace interface
{

namespace srv
{

struct SaveMaps
{
  using Request = interface::srv::SaveMaps_Request;
  using Response = interface::srv::SaveMaps_Response;
};

}  // namespace srv

}  // namespace interface

#endif  // INTERFACE__SRV__DETAIL__SAVE_MAPS__STRUCT_HPP_
