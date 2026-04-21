// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from interface:srv/IsValid.idl
// generated code does not contain a copyright notice

#ifndef INTERFACE__SRV__DETAIL__IS_VALID__STRUCT_HPP_
#define INTERFACE__SRV__DETAIL__IS_VALID__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__interface__srv__IsValid_Request __attribute__((deprecated))
#else
# define DEPRECATED__interface__srv__IsValid_Request __declspec(deprecated)
#endif

namespace interface
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct IsValid_Request_
{
  using Type = IsValid_Request_<ContainerAllocator>;

  explicit IsValid_Request_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->code = 0l;
    }
  }

  explicit IsValid_Request_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_alloc;
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->code = 0l;
    }
  }

  // field types and members
  using _code_type =
    int32_t;
  _code_type code;

  // setters for named parameter idiom
  Type & set__code(
    const int32_t & _arg)
  {
    this->code = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    interface::srv::IsValid_Request_<ContainerAllocator> *;
  using ConstRawPtr =
    const interface::srv::IsValid_Request_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<interface::srv::IsValid_Request_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<interface::srv::IsValid_Request_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      interface::srv::IsValid_Request_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<interface::srv::IsValid_Request_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      interface::srv::IsValid_Request_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<interface::srv::IsValid_Request_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<interface::srv::IsValid_Request_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<interface::srv::IsValid_Request_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__interface__srv__IsValid_Request
    std::shared_ptr<interface::srv::IsValid_Request_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__interface__srv__IsValid_Request
    std::shared_ptr<interface::srv::IsValid_Request_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const IsValid_Request_ & other) const
  {
    if (this->code != other.code) {
      return false;
    }
    return true;
  }
  bool operator!=(const IsValid_Request_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct IsValid_Request_

// alias to use template instance with default allocator
using IsValid_Request =
  interface::srv::IsValid_Request_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace interface


#ifndef _WIN32
# define DEPRECATED__interface__srv__IsValid_Response __attribute__((deprecated))
#else
# define DEPRECATED__interface__srv__IsValid_Response __declspec(deprecated)
#endif

namespace interface
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct IsValid_Response_
{
  using Type = IsValid_Response_<ContainerAllocator>;

  explicit IsValid_Response_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->valid = false;
    }
  }

  explicit IsValid_Response_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_alloc;
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->valid = false;
    }
  }

  // field types and members
  using _valid_type =
    bool;
  _valid_type valid;

  // setters for named parameter idiom
  Type & set__valid(
    const bool & _arg)
  {
    this->valid = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    interface::srv::IsValid_Response_<ContainerAllocator> *;
  using ConstRawPtr =
    const interface::srv::IsValid_Response_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<interface::srv::IsValid_Response_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<interface::srv::IsValid_Response_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      interface::srv::IsValid_Response_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<interface::srv::IsValid_Response_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      interface::srv::IsValid_Response_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<interface::srv::IsValid_Response_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<interface::srv::IsValid_Response_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<interface::srv::IsValid_Response_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__interface__srv__IsValid_Response
    std::shared_ptr<interface::srv::IsValid_Response_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__interface__srv__IsValid_Response
    std::shared_ptr<interface::srv::IsValid_Response_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const IsValid_Response_ & other) const
  {
    if (this->valid != other.valid) {
      return false;
    }
    return true;
  }
  bool operator!=(const IsValid_Response_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct IsValid_Response_

// alias to use template instance with default allocator
using IsValid_Response =
  interface::srv::IsValid_Response_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace interface

namespace interface
{

namespace srv
{

struct IsValid
{
  using Request = interface::srv::IsValid_Request;
  using Response = interface::srv::IsValid_Response;
};

}  // namespace srv

}  // namespace interface

#endif  // INTERFACE__SRV__DETAIL__IS_VALID__STRUCT_HPP_
