#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "teb_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__teb_msgs__msg__FeedbackMsg() -> *const std::ffi::c_void;
}

#[link(name = "teb_msgs__rosidl_generator_c")]
extern "C" {
    fn teb_msgs__msg__FeedbackMsg__init(msg: *mut FeedbackMsg) -> bool;
    fn teb_msgs__msg__FeedbackMsg__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FeedbackMsg>, size: usize) -> bool;
    fn teb_msgs__msg__FeedbackMsg__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FeedbackMsg>);
    fn teb_msgs__msg__FeedbackMsg__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FeedbackMsg>, out_seq: *mut rosidl_runtime_rs::Sequence<FeedbackMsg>) -> bool;
}

// Corresponds to teb_msgs__msg__FeedbackMsg
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Message that contains intermediate results 
/// and diagnostics of the (predictive) planner.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FeedbackMsg {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,

    /// The planned trajectory (or if multiple plans exist, all of them)
    pub trajectories: rosidl_runtime_rs::Sequence<super::super::msg::rmw::TrajectoryMsg>,

    /// Index of the trajectory in 'trajectories' that is selected currently
    pub selected_trajectory_idx: u16,

    /// List of active obstacles
    pub obstacles_msg: costmap_converter_msgs::msg::rmw::ObstacleArrayMsg,

}



impl Default for FeedbackMsg {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !teb_msgs__msg__FeedbackMsg__init(&mut msg as *mut _) {
        panic!("Call to teb_msgs__msg__FeedbackMsg__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FeedbackMsg {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { teb_msgs__msg__FeedbackMsg__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { teb_msgs__msg__FeedbackMsg__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { teb_msgs__msg__FeedbackMsg__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FeedbackMsg {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FeedbackMsg where Self: Sized {
  const TYPE_NAME: &'static str = "teb_msgs/msg/FeedbackMsg";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__teb_msgs__msg__FeedbackMsg() }
  }
}


#[link(name = "teb_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__teb_msgs__msg__TrajectoryMsg() -> *const std::ffi::c_void;
}

#[link(name = "teb_msgs__rosidl_generator_c")]
extern "C" {
    fn teb_msgs__msg__TrajectoryMsg__init(msg: *mut TrajectoryMsg) -> bool;
    fn teb_msgs__msg__TrajectoryMsg__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TrajectoryMsg>, size: usize) -> bool;
    fn teb_msgs__msg__TrajectoryMsg__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TrajectoryMsg>);
    fn teb_msgs__msg__TrajectoryMsg__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TrajectoryMsg>, out_seq: *mut rosidl_runtime_rs::Sequence<TrajectoryMsg>) -> bool;
}

// Corresponds to teb_msgs__msg__TrajectoryMsg
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Message that contains a trajectory for mobile robot navigation

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TrajectoryMsg {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub trajectory: rosidl_runtime_rs::Sequence<super::super::msg::rmw::TrajectoryPointMsg>,

}



impl Default for TrajectoryMsg {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !teb_msgs__msg__TrajectoryMsg__init(&mut msg as *mut _) {
        panic!("Call to teb_msgs__msg__TrajectoryMsg__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TrajectoryMsg {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { teb_msgs__msg__TrajectoryMsg__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { teb_msgs__msg__TrajectoryMsg__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { teb_msgs__msg__TrajectoryMsg__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TrajectoryMsg {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TrajectoryMsg where Self: Sized {
  const TYPE_NAME: &'static str = "teb_msgs/msg/TrajectoryMsg";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__teb_msgs__msg__TrajectoryMsg() }
  }
}


#[link(name = "teb_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__teb_msgs__msg__TrajectoryPointMsg() -> *const std::ffi::c_void;
}

#[link(name = "teb_msgs__rosidl_generator_c")]
extern "C" {
    fn teb_msgs__msg__TrajectoryPointMsg__init(msg: *mut TrajectoryPointMsg) -> bool;
    fn teb_msgs__msg__TrajectoryPointMsg__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TrajectoryPointMsg>, size: usize) -> bool;
    fn teb_msgs__msg__TrajectoryPointMsg__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TrajectoryPointMsg>);
    fn teb_msgs__msg__TrajectoryPointMsg__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TrajectoryPointMsg>, out_seq: *mut rosidl_runtime_rs::Sequence<TrajectoryPointMsg>) -> bool;
}

// Corresponds to teb_msgs__msg__TrajectoryPointMsg
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Message that contains single point on a trajectory suited for mobile navigation.
/// The trajectory is described by a sequence of poses, velocities,
/// accelerations and temporal information.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TrajectoryPointMsg {
    /// Why this message type?
    /// nav_msgs/Path describes only a path without temporal information.
    /// trajectory_msgs package contains only messages for joint trajectories.
    /// The pose of the robot
    pub pose: geometry_msgs::msg::rmw::Pose,

    /// Corresponding velocity
    pub velocity: geometry_msgs::msg::rmw::Twist,

    /// Corresponding acceleration
    pub acceleration: geometry_msgs::msg::rmw::Twist,


    // This member is not documented.
    #[allow(missing_docs)]
    pub time_from_start: builtin_interfaces::msg::rmw::Duration,

}



impl Default for TrajectoryPointMsg {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !teb_msgs__msg__TrajectoryPointMsg__init(&mut msg as *mut _) {
        panic!("Call to teb_msgs__msg__TrajectoryPointMsg__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TrajectoryPointMsg {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { teb_msgs__msg__TrajectoryPointMsg__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { teb_msgs__msg__TrajectoryPointMsg__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { teb_msgs__msg__TrajectoryPointMsg__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TrajectoryPointMsg {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TrajectoryPointMsg where Self: Sized {
  const TYPE_NAME: &'static str = "teb_msgs/msg/TrajectoryPointMsg";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__teb_msgs__msg__TrajectoryPointMsg() }
  }
}


