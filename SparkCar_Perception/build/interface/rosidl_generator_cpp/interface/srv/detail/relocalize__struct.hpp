// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from interface:srv/Relocalize.idl
// generated code does not contain a copyright notice

#ifndef INTERFACE__SRV__DETAIL__RELOCALIZE__STRUCT_HPP_
#define INTERFACE__SRV__DETAIL__RELOCALIZE__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__interface__srv__Relocalize_Request __attribute__((deprecated))
#else
# define DEPRECATED__interface__srv__Relocalize_Request __declspec(deprecated)
#endif

namespace interface
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct Relocalize_Request_
{
  using Type = Relocalize_Request_<ContainerAllocator>;

  explicit Relocalize_Request_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->pcd_path = "";
      this->x = 0.0f;
      this->y = 0.0f;
      this->z = 0.0f;
      this->yaw = 0.0f;
      this->pitch = 0.0f;
      this->roll = 0.0f;
    }
  }

  explicit Relocalize_Request_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : pcd_path(_alloc)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->pcd_path = "";
      this->x = 0.0f;
      this->y = 0.0f;
      this->z = 0.0f;
      this->yaw = 0.0f;
      this->pitch = 0.0f;
      this->roll = 0.0f;
    }
  }

  // field types and members
  using _pcd_path_type =
    std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>;
  _pcd_path_type pcd_path;
  using _x_type =
    float;
  _x_type x;
  using _y_type =
    float;
  _y_type y;
  using _z_type =
    float;
  _z_type z;
  using _yaw_type =
    float;
  _yaw_type yaw;
  using _pitch_type =
    float;
  _pitch_type pitch;
  using _roll_type =
    float;
  _roll_type roll;

  // setters for named parameter idiom
  Type & set__pcd_path(
    const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> & _arg)
  {
    this->pcd_path = _arg;
    return *this;
  }
  Type & set__x(
    const float & _arg)
  {
    this->x = _arg;
    return *this;
  }
  Type & set__y(
    const float & _arg)
  {
    this->y = _arg;
    return *this;
  }
  Type & set__z(
    const float & _arg)
  {
    this->z = _arg;
    return *this;
  }
  Type & set__yaw(
    const float & _arg)
  {
    this->yaw = _arg;
    return *this;
  }
  Type & set__pitch(
    const float & _arg)
  {
    this->pitch = _arg;
    return *this;
  }
  Type & set__roll(
    const float & _arg)
  {
    this->roll = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    interface::srv::Relocalize_Request_<ContainerAllocator> *;
  using ConstRawPtr =
    const interface::srv::Relocalize_Request_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<interface::srv::Relocalize_Request_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<interface::srv::Relocalize_Request_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      interface::srv::Relocalize_Request_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<interface::srv::Relocalize_Request_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      interface::srv::Relocalize_Request_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<interface::srv::Relocalize_Request_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<interface::srv::Relocalize_Request_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<interface::srv::Relocalize_Request_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__interface__srv__Relocalize_Request
    std::shared_ptr<interface::srv::Relocalize_Request_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__interface__srv__Relocalize_Request
    std::shared_ptr<interface::srv::Relocalize_Request_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const Relocalize_Request_ & other) const
  {
    if (this->pcd_path != other.pcd_path) {
      return false;
    }
    if (this->x != other.x) {
      return false;
    }
    if (this->y != other.y) {
      return false;
    }
    if (this->z != other.z) {
      return false;
    }
    if (this->yaw != other.yaw) {
      return false;
    }
    if (this->pitch != other.pitch) {
      return false;
    }
    if (this->roll != other.roll) {
      return false;
    }
    return true;
  }
  bool operator!=(const Relocalize_Request_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct Relocalize_Request_

// alias to use template instance with default allocator
using Relocalize_Request =
  interface::srv::Relocalize_Request_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace interface


#ifndef _WIN32
# define DEPRECATED__interface__srv__Relocalize_Response __attribute__((deprecated))
#else
# define DEPRECATED__interface__srv__Relocalize_Response __declspec(deprecated)
#endif

namespace interface
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct Relocalize_Response_
{
  using Type = Relocalize_Response_<ContainerAllocator>;

  explicit Relocalize_Response_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->success = false;
      this->message = "";
    }
  }

  explicit Relocalize_Response_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
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
    interface::srv::Relocalize_Response_<ContainerAllocator> *;
  using ConstRawPtr =
    const interface::srv::Relocalize_Response_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<interface::srv::Relocalize_Response_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<interface::srv::Relocalize_Response_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      interface::srv::Relocalize_Response_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<interface::srv::Relocalize_Response_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      interface::srv::Relocalize_Response_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<interface::srv::Relocalize_Response_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<interface::srv::Relocalize_Response_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<interface::srv::Relocalize_Response_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__interface__srv__Relocalize_Response
    std::shared_ptr<interface::srv::Relocalize_Response_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__interface__srv__Relocalize_Response
    std::shared_ptr<interface::srv::Relocalize_Response_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const Relocalize_Response_ & other) const
  {
    if (this->success != other.success) {
      return false;
    }
    if (this->message != other.message) {
      return false;
    }
    return true;
  }
  bool operator!=(const Relocalize_Response_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct Relocalize_Response_

// alias to use template instance with default allocator
using Relocalize_Response =
  interface::srv::Relocalize_Response_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace interface

namespace interface
{

namespace srv
{

struct Relocalize
{
  using Request = interface::srv::Relocalize_Request;
  using Response = interface::srv::Relocalize_Response;
};

}  // namespace srv

}  // namespace interface

#endif  // INTERFACE__SRV__DETAIL__RELOCALIZE__STRUCT_HPP_
