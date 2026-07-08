#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "costmap_converter_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__costmap_converter_msgs__msg__ObstacleMsg() -> *const std::ffi::c_void;
}

#[link(name = "costmap_converter_msgs__rosidl_generator_c")]
extern "C" {
    fn costmap_converter_msgs__msg__ObstacleMsg__init(msg: *mut ObstacleMsg) -> bool;
    fn costmap_converter_msgs__msg__ObstacleMsg__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ObstacleMsg>, size: usize) -> bool;
    fn costmap_converter_msgs__msg__ObstacleMsg__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ObstacleMsg>);
    fn costmap_converter_msgs__msg__ObstacleMsg__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ObstacleMsg>, out_seq: *mut rosidl_runtime_rs::Sequence<ObstacleMsg>) -> bool;
}

// Corresponds to costmap_converter_msgs__msg__ObstacleMsg
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Special types:
/// Polygon with 1 vertex: Point obstacle (you might also specify a non-zero value for radius)
/// Polygon with 2 vertices: Line obstacle
/// Polygon with more than 2 vertices: First and last points are assumed to be connected

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ObstacleMsg {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,

    /// Obstacle footprint (polygon descriptions)
    pub polygon: geometry_msgs::msg::rmw::Polygon,

    /// Specify the radius for circular/point obstacles
    pub radius: f64,

    /// Obstacle ID
    /// Specify IDs in order to provide (temporal) relationships
    /// between obstacles among multiple messages.
    pub id: i64,

    /// Individual orientation (centroid)
    pub orientation: geometry_msgs::msg::rmw::Quaternion,

    /// Individual velocities (centroid)
    pub velocities: geometry_msgs::msg::rmw::TwistWithCovariance,

}



impl Default for ObstacleMsg {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !costmap_converter_msgs__msg__ObstacleMsg__init(&mut msg as *mut _) {
        panic!("Call to costmap_converter_msgs__msg__ObstacleMsg__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ObstacleMsg {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { costmap_converter_msgs__msg__ObstacleMsg__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { costmap_converter_msgs__msg__ObstacleMsg__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { costmap_converter_msgs__msg__ObstacleMsg__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ObstacleMsg {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ObstacleMsg where Self: Sized {
  const TYPE_NAME: &'static str = "costmap_converter_msgs/msg/ObstacleMsg";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__costmap_converter_msgs__msg__ObstacleMsg() }
  }
}


#[link(name = "costmap_converter_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__costmap_converter_msgs__msg__ObstacleArrayMsg() -> *const std::ffi::c_void;
}

#[link(name = "costmap_converter_msgs__rosidl_generator_c")]
extern "C" {
    fn costmap_converter_msgs__msg__ObstacleArrayMsg__init(msg: *mut ObstacleArrayMsg) -> bool;
    fn costmap_converter_msgs__msg__ObstacleArrayMsg__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ObstacleArrayMsg>, size: usize) -> bool;
    fn costmap_converter_msgs__msg__ObstacleArrayMsg__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ObstacleArrayMsg>);
    fn costmap_converter_msgs__msg__ObstacleArrayMsg__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ObstacleArrayMsg>, out_seq: *mut rosidl_runtime_rs::Sequence<ObstacleArrayMsg>) -> bool;
}

// Corresponds to costmap_converter_msgs__msg__ObstacleArrayMsg
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Message that contains a list of polygon shaped obstacles.
/// Special types:
/// Polygon with 1 vertex: Point obstacle
/// Polygon with 2 vertices: Line obstacle
/// Polygon with more than 2 vertices: First and last points are assumed to be connected

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ObstacleArrayMsg {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub obstacles: rosidl_runtime_rs::Sequence<super::super::msg::rmw::ObstacleMsg>,

}



impl Default for ObstacleArrayMsg {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !costmap_converter_msgs__msg__ObstacleArrayMsg__init(&mut msg as *mut _) {
        panic!("Call to costmap_converter_msgs__msg__ObstacleArrayMsg__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ObstacleArrayMsg {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { costmap_converter_msgs__msg__ObstacleArrayMsg__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { costmap_converter_msgs__msg__ObstacleArrayMsg__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { costmap_converter_msgs__msg__ObstacleArrayMsg__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ObstacleArrayMsg {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ObstacleArrayMsg where Self: Sized {
  const TYPE_NAME: &'static str = "costmap_converter_msgs/msg/ObstacleArrayMsg";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__costmap_converter_msgs__msg__ObstacleArrayMsg() }
  }
}


