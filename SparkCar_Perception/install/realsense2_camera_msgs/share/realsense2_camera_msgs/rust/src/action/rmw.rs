
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "realsense2_camera_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__action__TriggeredCalibration_Goal() -> *const std::ffi::c_void;
}

#[link(name = "realsense2_camera_msgs__rosidl_generator_c")]
extern "C" {
    fn realsense2_camera_msgs__action__TriggeredCalibration_Goal__init(msg: *mut TriggeredCalibration_Goal) -> bool;
    fn realsense2_camera_msgs__action__TriggeredCalibration_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TriggeredCalibration_Goal>, size: usize) -> bool;
    fn realsense2_camera_msgs__action__TriggeredCalibration_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TriggeredCalibration_Goal>);
    fn realsense2_camera_msgs__action__TriggeredCalibration_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TriggeredCalibration_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<TriggeredCalibration_Goal>) -> bool;
}

// Corresponds to realsense2_camera_msgs__action__TriggeredCalibration_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TriggeredCalibration_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub json: rosidl_runtime_rs::String,

}



impl Default for TriggeredCalibration_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !realsense2_camera_msgs__action__TriggeredCalibration_Goal__init(&mut msg as *mut _) {
        panic!("Call to realsense2_camera_msgs__action__TriggeredCalibration_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TriggeredCalibration_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__action__TriggeredCalibration_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__action__TriggeredCalibration_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__action__TriggeredCalibration_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TriggeredCalibration_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TriggeredCalibration_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "realsense2_camera_msgs/action/TriggeredCalibration_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__action__TriggeredCalibration_Goal() }
  }
}


#[link(name = "realsense2_camera_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__action__TriggeredCalibration_Result() -> *const std::ffi::c_void;
}

#[link(name = "realsense2_camera_msgs__rosidl_generator_c")]
extern "C" {
    fn realsense2_camera_msgs__action__TriggeredCalibration_Result__init(msg: *mut TriggeredCalibration_Result) -> bool;
    fn realsense2_camera_msgs__action__TriggeredCalibration_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TriggeredCalibration_Result>, size: usize) -> bool;
    fn realsense2_camera_msgs__action__TriggeredCalibration_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TriggeredCalibration_Result>);
    fn realsense2_camera_msgs__action__TriggeredCalibration_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TriggeredCalibration_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<TriggeredCalibration_Result>) -> bool;
}

// Corresponds to realsense2_camera_msgs__action__TriggeredCalibration_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TriggeredCalibration_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_msg: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub calibration: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub health: f32,

}



impl Default for TriggeredCalibration_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !realsense2_camera_msgs__action__TriggeredCalibration_Result__init(&mut msg as *mut _) {
        panic!("Call to realsense2_camera_msgs__action__TriggeredCalibration_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TriggeredCalibration_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__action__TriggeredCalibration_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__action__TriggeredCalibration_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__action__TriggeredCalibration_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TriggeredCalibration_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TriggeredCalibration_Result where Self: Sized {
  const TYPE_NAME: &'static str = "realsense2_camera_msgs/action/TriggeredCalibration_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__action__TriggeredCalibration_Result() }
  }
}


#[link(name = "realsense2_camera_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__action__TriggeredCalibration_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "realsense2_camera_msgs__rosidl_generator_c")]
extern "C" {
    fn realsense2_camera_msgs__action__TriggeredCalibration_Feedback__init(msg: *mut TriggeredCalibration_Feedback) -> bool;
    fn realsense2_camera_msgs__action__TriggeredCalibration_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TriggeredCalibration_Feedback>, size: usize) -> bool;
    fn realsense2_camera_msgs__action__TriggeredCalibration_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TriggeredCalibration_Feedback>);
    fn realsense2_camera_msgs__action__TriggeredCalibration_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TriggeredCalibration_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<TriggeredCalibration_Feedback>) -> bool;
}

// Corresponds to realsense2_camera_msgs__action__TriggeredCalibration_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TriggeredCalibration_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub progress: f32,

}



impl Default for TriggeredCalibration_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !realsense2_camera_msgs__action__TriggeredCalibration_Feedback__init(&mut msg as *mut _) {
        panic!("Call to realsense2_camera_msgs__action__TriggeredCalibration_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TriggeredCalibration_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__action__TriggeredCalibration_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__action__TriggeredCalibration_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__action__TriggeredCalibration_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TriggeredCalibration_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TriggeredCalibration_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "realsense2_camera_msgs/action/TriggeredCalibration_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__action__TriggeredCalibration_Feedback() }
  }
}


#[link(name = "realsense2_camera_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__action__TriggeredCalibration_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "realsense2_camera_msgs__rosidl_generator_c")]
extern "C" {
    fn realsense2_camera_msgs__action__TriggeredCalibration_FeedbackMessage__init(msg: *mut TriggeredCalibration_FeedbackMessage) -> bool;
    fn realsense2_camera_msgs__action__TriggeredCalibration_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TriggeredCalibration_FeedbackMessage>, size: usize) -> bool;
    fn realsense2_camera_msgs__action__TriggeredCalibration_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TriggeredCalibration_FeedbackMessage>);
    fn realsense2_camera_msgs__action__TriggeredCalibration_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TriggeredCalibration_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<TriggeredCalibration_FeedbackMessage>) -> bool;
}

// Corresponds to realsense2_camera_msgs__action__TriggeredCalibration_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TriggeredCalibration_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::TriggeredCalibration_Feedback,

}



impl Default for TriggeredCalibration_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !realsense2_camera_msgs__action__TriggeredCalibration_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to realsense2_camera_msgs__action__TriggeredCalibration_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TriggeredCalibration_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__action__TriggeredCalibration_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__action__TriggeredCalibration_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__action__TriggeredCalibration_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TriggeredCalibration_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TriggeredCalibration_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "realsense2_camera_msgs/action/TriggeredCalibration_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__action__TriggeredCalibration_FeedbackMessage() }
  }
}




#[link(name = "realsense2_camera_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__action__TriggeredCalibration_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "realsense2_camera_msgs__rosidl_generator_c")]
extern "C" {
    fn realsense2_camera_msgs__action__TriggeredCalibration_SendGoal_Request__init(msg: *mut TriggeredCalibration_SendGoal_Request) -> bool;
    fn realsense2_camera_msgs__action__TriggeredCalibration_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TriggeredCalibration_SendGoal_Request>, size: usize) -> bool;
    fn realsense2_camera_msgs__action__TriggeredCalibration_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TriggeredCalibration_SendGoal_Request>);
    fn realsense2_camera_msgs__action__TriggeredCalibration_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TriggeredCalibration_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<TriggeredCalibration_SendGoal_Request>) -> bool;
}

// Corresponds to realsense2_camera_msgs__action__TriggeredCalibration_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TriggeredCalibration_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::TriggeredCalibration_Goal,

}



impl Default for TriggeredCalibration_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !realsense2_camera_msgs__action__TriggeredCalibration_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to realsense2_camera_msgs__action__TriggeredCalibration_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TriggeredCalibration_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__action__TriggeredCalibration_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__action__TriggeredCalibration_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__action__TriggeredCalibration_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TriggeredCalibration_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TriggeredCalibration_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "realsense2_camera_msgs/action/TriggeredCalibration_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__action__TriggeredCalibration_SendGoal_Request() }
  }
}


#[link(name = "realsense2_camera_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__action__TriggeredCalibration_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "realsense2_camera_msgs__rosidl_generator_c")]
extern "C" {
    fn realsense2_camera_msgs__action__TriggeredCalibration_SendGoal_Response__init(msg: *mut TriggeredCalibration_SendGoal_Response) -> bool;
    fn realsense2_camera_msgs__action__TriggeredCalibration_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TriggeredCalibration_SendGoal_Response>, size: usize) -> bool;
    fn realsense2_camera_msgs__action__TriggeredCalibration_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TriggeredCalibration_SendGoal_Response>);
    fn realsense2_camera_msgs__action__TriggeredCalibration_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TriggeredCalibration_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<TriggeredCalibration_SendGoal_Response>) -> bool;
}

// Corresponds to realsense2_camera_msgs__action__TriggeredCalibration_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TriggeredCalibration_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for TriggeredCalibration_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !realsense2_camera_msgs__action__TriggeredCalibration_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to realsense2_camera_msgs__action__TriggeredCalibration_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TriggeredCalibration_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__action__TriggeredCalibration_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__action__TriggeredCalibration_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__action__TriggeredCalibration_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TriggeredCalibration_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TriggeredCalibration_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "realsense2_camera_msgs/action/TriggeredCalibration_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__action__TriggeredCalibration_SendGoal_Response() }
  }
}


#[link(name = "realsense2_camera_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__action__TriggeredCalibration_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "realsense2_camera_msgs__rosidl_generator_c")]
extern "C" {
    fn realsense2_camera_msgs__action__TriggeredCalibration_GetResult_Request__init(msg: *mut TriggeredCalibration_GetResult_Request) -> bool;
    fn realsense2_camera_msgs__action__TriggeredCalibration_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TriggeredCalibration_GetResult_Request>, size: usize) -> bool;
    fn realsense2_camera_msgs__action__TriggeredCalibration_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TriggeredCalibration_GetResult_Request>);
    fn realsense2_camera_msgs__action__TriggeredCalibration_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TriggeredCalibration_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<TriggeredCalibration_GetResult_Request>) -> bool;
}

// Corresponds to realsense2_camera_msgs__action__TriggeredCalibration_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TriggeredCalibration_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for TriggeredCalibration_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !realsense2_camera_msgs__action__TriggeredCalibration_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to realsense2_camera_msgs__action__TriggeredCalibration_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TriggeredCalibration_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__action__TriggeredCalibration_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__action__TriggeredCalibration_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__action__TriggeredCalibration_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TriggeredCalibration_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TriggeredCalibration_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "realsense2_camera_msgs/action/TriggeredCalibration_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__action__TriggeredCalibration_GetResult_Request() }
  }
}


#[link(name = "realsense2_camera_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__action__TriggeredCalibration_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "realsense2_camera_msgs__rosidl_generator_c")]
extern "C" {
    fn realsense2_camera_msgs__action__TriggeredCalibration_GetResult_Response__init(msg: *mut TriggeredCalibration_GetResult_Response) -> bool;
    fn realsense2_camera_msgs__action__TriggeredCalibration_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TriggeredCalibration_GetResult_Response>, size: usize) -> bool;
    fn realsense2_camera_msgs__action__TriggeredCalibration_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TriggeredCalibration_GetResult_Response>);
    fn realsense2_camera_msgs__action__TriggeredCalibration_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TriggeredCalibration_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<TriggeredCalibration_GetResult_Response>) -> bool;
}

// Corresponds to realsense2_camera_msgs__action__TriggeredCalibration_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TriggeredCalibration_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::TriggeredCalibration_Result,

}



impl Default for TriggeredCalibration_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !realsense2_camera_msgs__action__TriggeredCalibration_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to realsense2_camera_msgs__action__TriggeredCalibration_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TriggeredCalibration_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__action__TriggeredCalibration_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__action__TriggeredCalibration_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__action__TriggeredCalibration_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TriggeredCalibration_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TriggeredCalibration_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "realsense2_camera_msgs/action/TriggeredCalibration_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__action__TriggeredCalibration_GetResult_Response() }
  }
}






#[link(name = "realsense2_camera_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__realsense2_camera_msgs__action__TriggeredCalibration_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to realsense2_camera_msgs__action__TriggeredCalibration_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct TriggeredCalibration_SendGoal;

impl rosidl_runtime_rs::Service for TriggeredCalibration_SendGoal {
    type Request = TriggeredCalibration_SendGoal_Request;
    type Response = TriggeredCalibration_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__realsense2_camera_msgs__action__TriggeredCalibration_SendGoal() }
    }
}




#[link(name = "realsense2_camera_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__realsense2_camera_msgs__action__TriggeredCalibration_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to realsense2_camera_msgs__action__TriggeredCalibration_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct TriggeredCalibration_GetResult;

impl rosidl_runtime_rs::Service for TriggeredCalibration_GetResult {
    type Request = TriggeredCalibration_GetResult_Request;
    type Response = TriggeredCalibration_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__realsense2_camera_msgs__action__TriggeredCalibration_GetResult() }
    }
}


