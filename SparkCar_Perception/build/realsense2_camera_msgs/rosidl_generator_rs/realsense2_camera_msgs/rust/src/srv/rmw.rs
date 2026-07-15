#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "realsense2_camera_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__DeviceInfo_Request() -> *const std::ffi::c_void;
}

#[link(name = "realsense2_camera_msgs__rosidl_generator_c")]
extern "C" {
    fn realsense2_camera_msgs__srv__DeviceInfo_Request__init(msg: *mut DeviceInfo_Request) -> bool;
    fn realsense2_camera_msgs__srv__DeviceInfo_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DeviceInfo_Request>, size: usize) -> bool;
    fn realsense2_camera_msgs__srv__DeviceInfo_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DeviceInfo_Request>);
    fn realsense2_camera_msgs__srv__DeviceInfo_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DeviceInfo_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<DeviceInfo_Request>) -> bool;
}

// Corresponds to realsense2_camera_msgs__srv__DeviceInfo_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DeviceInfo_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for DeviceInfo_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !realsense2_camera_msgs__srv__DeviceInfo_Request__init(&mut msg as *mut _) {
        panic!("Call to realsense2_camera_msgs__srv__DeviceInfo_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DeviceInfo_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__DeviceInfo_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__DeviceInfo_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__DeviceInfo_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DeviceInfo_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DeviceInfo_Request where Self: Sized {
  const TYPE_NAME: &'static str = "realsense2_camera_msgs/srv/DeviceInfo_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__DeviceInfo_Request() }
  }
}


#[link(name = "realsense2_camera_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__DeviceInfo_Response() -> *const std::ffi::c_void;
}

#[link(name = "realsense2_camera_msgs__rosidl_generator_c")]
extern "C" {
    fn realsense2_camera_msgs__srv__DeviceInfo_Response__init(msg: *mut DeviceInfo_Response) -> bool;
    fn realsense2_camera_msgs__srv__DeviceInfo_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DeviceInfo_Response>, size: usize) -> bool;
    fn realsense2_camera_msgs__srv__DeviceInfo_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DeviceInfo_Response>);
    fn realsense2_camera_msgs__srv__DeviceInfo_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DeviceInfo_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<DeviceInfo_Response>) -> bool;
}

// Corresponds to realsense2_camera_msgs__srv__DeviceInfo_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DeviceInfo_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub device_name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub serial_number: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub firmware_version: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub usb_type_descriptor: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub firmware_update_id: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub sensors: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub physical_port: rosidl_runtime_rs::String,

}



impl Default for DeviceInfo_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !realsense2_camera_msgs__srv__DeviceInfo_Response__init(&mut msg as *mut _) {
        panic!("Call to realsense2_camera_msgs__srv__DeviceInfo_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DeviceInfo_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__DeviceInfo_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__DeviceInfo_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__DeviceInfo_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DeviceInfo_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DeviceInfo_Response where Self: Sized {
  const TYPE_NAME: &'static str = "realsense2_camera_msgs/srv/DeviceInfo_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__DeviceInfo_Response() }
  }
}


#[link(name = "realsense2_camera_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__SafetyPresetRead_Request() -> *const std::ffi::c_void;
}

#[link(name = "realsense2_camera_msgs__rosidl_generator_c")]
extern "C" {
    fn realsense2_camera_msgs__srv__SafetyPresetRead_Request__init(msg: *mut SafetyPresetRead_Request) -> bool;
    fn realsense2_camera_msgs__srv__SafetyPresetRead_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SafetyPresetRead_Request>, size: usize) -> bool;
    fn realsense2_camera_msgs__srv__SafetyPresetRead_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SafetyPresetRead_Request>);
    fn realsense2_camera_msgs__srv__SafetyPresetRead_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SafetyPresetRead_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SafetyPresetRead_Request>) -> bool;
}

// Corresponds to realsense2_camera_msgs__srv__SafetyPresetRead_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SafetyPresetRead_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: u8,

}



impl Default for SafetyPresetRead_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !realsense2_camera_msgs__srv__SafetyPresetRead_Request__init(&mut msg as *mut _) {
        panic!("Call to realsense2_camera_msgs__srv__SafetyPresetRead_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SafetyPresetRead_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__SafetyPresetRead_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__SafetyPresetRead_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__SafetyPresetRead_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SafetyPresetRead_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SafetyPresetRead_Request where Self: Sized {
  const TYPE_NAME: &'static str = "realsense2_camera_msgs/srv/SafetyPresetRead_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__SafetyPresetRead_Request() }
  }
}


#[link(name = "realsense2_camera_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__SafetyPresetRead_Response() -> *const std::ffi::c_void;
}

#[link(name = "realsense2_camera_msgs__rosidl_generator_c")]
extern "C" {
    fn realsense2_camera_msgs__srv__SafetyPresetRead_Response__init(msg: *mut SafetyPresetRead_Response) -> bool;
    fn realsense2_camera_msgs__srv__SafetyPresetRead_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SafetyPresetRead_Response>, size: usize) -> bool;
    fn realsense2_camera_msgs__srv__SafetyPresetRead_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SafetyPresetRead_Response>);
    fn realsense2_camera_msgs__srv__SafetyPresetRead_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SafetyPresetRead_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SafetyPresetRead_Response>) -> bool;
}

// Corresponds to realsense2_camera_msgs__srv__SafetyPresetRead_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SafetyPresetRead_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_message: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub safety_preset: rosidl_runtime_rs::String,

}



impl Default for SafetyPresetRead_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !realsense2_camera_msgs__srv__SafetyPresetRead_Response__init(&mut msg as *mut _) {
        panic!("Call to realsense2_camera_msgs__srv__SafetyPresetRead_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SafetyPresetRead_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__SafetyPresetRead_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__SafetyPresetRead_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__SafetyPresetRead_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SafetyPresetRead_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SafetyPresetRead_Response where Self: Sized {
  const TYPE_NAME: &'static str = "realsense2_camera_msgs/srv/SafetyPresetRead_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__SafetyPresetRead_Response() }
  }
}


#[link(name = "realsense2_camera_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__SafetyPresetWrite_Request() -> *const std::ffi::c_void;
}

#[link(name = "realsense2_camera_msgs__rosidl_generator_c")]
extern "C" {
    fn realsense2_camera_msgs__srv__SafetyPresetWrite_Request__init(msg: *mut SafetyPresetWrite_Request) -> bool;
    fn realsense2_camera_msgs__srv__SafetyPresetWrite_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SafetyPresetWrite_Request>, size: usize) -> bool;
    fn realsense2_camera_msgs__srv__SafetyPresetWrite_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SafetyPresetWrite_Request>);
    fn realsense2_camera_msgs__srv__SafetyPresetWrite_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SafetyPresetWrite_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SafetyPresetWrite_Request>) -> bool;
}

// Corresponds to realsense2_camera_msgs__srv__SafetyPresetWrite_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SafetyPresetWrite_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub safety_preset: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub index: u8,

}



impl Default for SafetyPresetWrite_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !realsense2_camera_msgs__srv__SafetyPresetWrite_Request__init(&mut msg as *mut _) {
        panic!("Call to realsense2_camera_msgs__srv__SafetyPresetWrite_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SafetyPresetWrite_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__SafetyPresetWrite_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__SafetyPresetWrite_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__SafetyPresetWrite_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SafetyPresetWrite_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SafetyPresetWrite_Request where Self: Sized {
  const TYPE_NAME: &'static str = "realsense2_camera_msgs/srv/SafetyPresetWrite_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__SafetyPresetWrite_Request() }
  }
}


#[link(name = "realsense2_camera_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__SafetyPresetWrite_Response() -> *const std::ffi::c_void;
}

#[link(name = "realsense2_camera_msgs__rosidl_generator_c")]
extern "C" {
    fn realsense2_camera_msgs__srv__SafetyPresetWrite_Response__init(msg: *mut SafetyPresetWrite_Response) -> bool;
    fn realsense2_camera_msgs__srv__SafetyPresetWrite_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SafetyPresetWrite_Response>, size: usize) -> bool;
    fn realsense2_camera_msgs__srv__SafetyPresetWrite_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SafetyPresetWrite_Response>);
    fn realsense2_camera_msgs__srv__SafetyPresetWrite_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SafetyPresetWrite_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SafetyPresetWrite_Response>) -> bool;
}

// Corresponds to realsense2_camera_msgs__srv__SafetyPresetWrite_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SafetyPresetWrite_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_message: rosidl_runtime_rs::String,

}



impl Default for SafetyPresetWrite_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !realsense2_camera_msgs__srv__SafetyPresetWrite_Response__init(&mut msg as *mut _) {
        panic!("Call to realsense2_camera_msgs__srv__SafetyPresetWrite_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SafetyPresetWrite_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__SafetyPresetWrite_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__SafetyPresetWrite_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__SafetyPresetWrite_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SafetyPresetWrite_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SafetyPresetWrite_Response where Self: Sized {
  const TYPE_NAME: &'static str = "realsense2_camera_msgs/srv/SafetyPresetWrite_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__SafetyPresetWrite_Response() }
  }
}


#[link(name = "realsense2_camera_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__SafetyInterfaceConfigRead_Request() -> *const std::ffi::c_void;
}

#[link(name = "realsense2_camera_msgs__rosidl_generator_c")]
extern "C" {
    fn realsense2_camera_msgs__srv__SafetyInterfaceConfigRead_Request__init(msg: *mut SafetyInterfaceConfigRead_Request) -> bool;
    fn realsense2_camera_msgs__srv__SafetyInterfaceConfigRead_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SafetyInterfaceConfigRead_Request>, size: usize) -> bool;
    fn realsense2_camera_msgs__srv__SafetyInterfaceConfigRead_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SafetyInterfaceConfigRead_Request>);
    fn realsense2_camera_msgs__srv__SafetyInterfaceConfigRead_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SafetyInterfaceConfigRead_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SafetyInterfaceConfigRead_Request>) -> bool;
}

// Corresponds to realsense2_camera_msgs__srv__SafetyInterfaceConfigRead_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SafetyInterfaceConfigRead_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub calib_location: u8,

}



impl Default for SafetyInterfaceConfigRead_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !realsense2_camera_msgs__srv__SafetyInterfaceConfigRead_Request__init(&mut msg as *mut _) {
        panic!("Call to realsense2_camera_msgs__srv__SafetyInterfaceConfigRead_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SafetyInterfaceConfigRead_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__SafetyInterfaceConfigRead_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__SafetyInterfaceConfigRead_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__SafetyInterfaceConfigRead_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SafetyInterfaceConfigRead_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SafetyInterfaceConfigRead_Request where Self: Sized {
  const TYPE_NAME: &'static str = "realsense2_camera_msgs/srv/SafetyInterfaceConfigRead_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__SafetyInterfaceConfigRead_Request() }
  }
}


#[link(name = "realsense2_camera_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__SafetyInterfaceConfigRead_Response() -> *const std::ffi::c_void;
}

#[link(name = "realsense2_camera_msgs__rosidl_generator_c")]
extern "C" {
    fn realsense2_camera_msgs__srv__SafetyInterfaceConfigRead_Response__init(msg: *mut SafetyInterfaceConfigRead_Response) -> bool;
    fn realsense2_camera_msgs__srv__SafetyInterfaceConfigRead_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SafetyInterfaceConfigRead_Response>, size: usize) -> bool;
    fn realsense2_camera_msgs__srv__SafetyInterfaceConfigRead_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SafetyInterfaceConfigRead_Response>);
    fn realsense2_camera_msgs__srv__SafetyInterfaceConfigRead_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SafetyInterfaceConfigRead_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SafetyInterfaceConfigRead_Response>) -> bool;
}

// Corresponds to realsense2_camera_msgs__srv__SafetyInterfaceConfigRead_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SafetyInterfaceConfigRead_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_message: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub safety_interface_config: rosidl_runtime_rs::String,

}



impl Default for SafetyInterfaceConfigRead_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !realsense2_camera_msgs__srv__SafetyInterfaceConfigRead_Response__init(&mut msg as *mut _) {
        panic!("Call to realsense2_camera_msgs__srv__SafetyInterfaceConfigRead_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SafetyInterfaceConfigRead_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__SafetyInterfaceConfigRead_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__SafetyInterfaceConfigRead_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__SafetyInterfaceConfigRead_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SafetyInterfaceConfigRead_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SafetyInterfaceConfigRead_Response where Self: Sized {
  const TYPE_NAME: &'static str = "realsense2_camera_msgs/srv/SafetyInterfaceConfigRead_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__SafetyInterfaceConfigRead_Response() }
  }
}


#[link(name = "realsense2_camera_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__SafetyInterfaceConfigWrite_Request() -> *const std::ffi::c_void;
}

#[link(name = "realsense2_camera_msgs__rosidl_generator_c")]
extern "C" {
    fn realsense2_camera_msgs__srv__SafetyInterfaceConfigWrite_Request__init(msg: *mut SafetyInterfaceConfigWrite_Request) -> bool;
    fn realsense2_camera_msgs__srv__SafetyInterfaceConfigWrite_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SafetyInterfaceConfigWrite_Request>, size: usize) -> bool;
    fn realsense2_camera_msgs__srv__SafetyInterfaceConfigWrite_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SafetyInterfaceConfigWrite_Request>);
    fn realsense2_camera_msgs__srv__SafetyInterfaceConfigWrite_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SafetyInterfaceConfigWrite_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SafetyInterfaceConfigWrite_Request>) -> bool;
}

// Corresponds to realsense2_camera_msgs__srv__SafetyInterfaceConfigWrite_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SafetyInterfaceConfigWrite_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub safety_interface_config: rosidl_runtime_rs::String,

}



impl Default for SafetyInterfaceConfigWrite_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !realsense2_camera_msgs__srv__SafetyInterfaceConfigWrite_Request__init(&mut msg as *mut _) {
        panic!("Call to realsense2_camera_msgs__srv__SafetyInterfaceConfigWrite_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SafetyInterfaceConfigWrite_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__SafetyInterfaceConfigWrite_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__SafetyInterfaceConfigWrite_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__SafetyInterfaceConfigWrite_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SafetyInterfaceConfigWrite_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SafetyInterfaceConfigWrite_Request where Self: Sized {
  const TYPE_NAME: &'static str = "realsense2_camera_msgs/srv/SafetyInterfaceConfigWrite_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__SafetyInterfaceConfigWrite_Request() }
  }
}


#[link(name = "realsense2_camera_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__SafetyInterfaceConfigWrite_Response() -> *const std::ffi::c_void;
}

#[link(name = "realsense2_camera_msgs__rosidl_generator_c")]
extern "C" {
    fn realsense2_camera_msgs__srv__SafetyInterfaceConfigWrite_Response__init(msg: *mut SafetyInterfaceConfigWrite_Response) -> bool;
    fn realsense2_camera_msgs__srv__SafetyInterfaceConfigWrite_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SafetyInterfaceConfigWrite_Response>, size: usize) -> bool;
    fn realsense2_camera_msgs__srv__SafetyInterfaceConfigWrite_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SafetyInterfaceConfigWrite_Response>);
    fn realsense2_camera_msgs__srv__SafetyInterfaceConfigWrite_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SafetyInterfaceConfigWrite_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SafetyInterfaceConfigWrite_Response>) -> bool;
}

// Corresponds to realsense2_camera_msgs__srv__SafetyInterfaceConfigWrite_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SafetyInterfaceConfigWrite_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_message: rosidl_runtime_rs::String,

}



impl Default for SafetyInterfaceConfigWrite_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !realsense2_camera_msgs__srv__SafetyInterfaceConfigWrite_Response__init(&mut msg as *mut _) {
        panic!("Call to realsense2_camera_msgs__srv__SafetyInterfaceConfigWrite_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SafetyInterfaceConfigWrite_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__SafetyInterfaceConfigWrite_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__SafetyInterfaceConfigWrite_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__SafetyInterfaceConfigWrite_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SafetyInterfaceConfigWrite_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SafetyInterfaceConfigWrite_Response where Self: Sized {
  const TYPE_NAME: &'static str = "realsense2_camera_msgs/srv/SafetyInterfaceConfigWrite_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__SafetyInterfaceConfigWrite_Response() }
  }
}


#[link(name = "realsense2_camera_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__CalibConfigRead_Request() -> *const std::ffi::c_void;
}

#[link(name = "realsense2_camera_msgs__rosidl_generator_c")]
extern "C" {
    fn realsense2_camera_msgs__srv__CalibConfigRead_Request__init(msg: *mut CalibConfigRead_Request) -> bool;
    fn realsense2_camera_msgs__srv__CalibConfigRead_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CalibConfigRead_Request>, size: usize) -> bool;
    fn realsense2_camera_msgs__srv__CalibConfigRead_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CalibConfigRead_Request>);
    fn realsense2_camera_msgs__srv__CalibConfigRead_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CalibConfigRead_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<CalibConfigRead_Request>) -> bool;
}

// Corresponds to realsense2_camera_msgs__srv__CalibConfigRead_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CalibConfigRead_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for CalibConfigRead_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !realsense2_camera_msgs__srv__CalibConfigRead_Request__init(&mut msg as *mut _) {
        panic!("Call to realsense2_camera_msgs__srv__CalibConfigRead_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CalibConfigRead_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__CalibConfigRead_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__CalibConfigRead_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__CalibConfigRead_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CalibConfigRead_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CalibConfigRead_Request where Self: Sized {
  const TYPE_NAME: &'static str = "realsense2_camera_msgs/srv/CalibConfigRead_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__CalibConfigRead_Request() }
  }
}


#[link(name = "realsense2_camera_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__CalibConfigRead_Response() -> *const std::ffi::c_void;
}

#[link(name = "realsense2_camera_msgs__rosidl_generator_c")]
extern "C" {
    fn realsense2_camera_msgs__srv__CalibConfigRead_Response__init(msg: *mut CalibConfigRead_Response) -> bool;
    fn realsense2_camera_msgs__srv__CalibConfigRead_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CalibConfigRead_Response>, size: usize) -> bool;
    fn realsense2_camera_msgs__srv__CalibConfigRead_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CalibConfigRead_Response>);
    fn realsense2_camera_msgs__srv__CalibConfigRead_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CalibConfigRead_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<CalibConfigRead_Response>) -> bool;
}

// Corresponds to realsense2_camera_msgs__srv__CalibConfigRead_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CalibConfigRead_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_message: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub calib_config: rosidl_runtime_rs::String,

}



impl Default for CalibConfigRead_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !realsense2_camera_msgs__srv__CalibConfigRead_Response__init(&mut msg as *mut _) {
        panic!("Call to realsense2_camera_msgs__srv__CalibConfigRead_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CalibConfigRead_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__CalibConfigRead_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__CalibConfigRead_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__CalibConfigRead_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CalibConfigRead_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CalibConfigRead_Response where Self: Sized {
  const TYPE_NAME: &'static str = "realsense2_camera_msgs/srv/CalibConfigRead_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__CalibConfigRead_Response() }
  }
}


#[link(name = "realsense2_camera_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__CalibConfigWrite_Request() -> *const std::ffi::c_void;
}

#[link(name = "realsense2_camera_msgs__rosidl_generator_c")]
extern "C" {
    fn realsense2_camera_msgs__srv__CalibConfigWrite_Request__init(msg: *mut CalibConfigWrite_Request) -> bool;
    fn realsense2_camera_msgs__srv__CalibConfigWrite_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CalibConfigWrite_Request>, size: usize) -> bool;
    fn realsense2_camera_msgs__srv__CalibConfigWrite_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CalibConfigWrite_Request>);
    fn realsense2_camera_msgs__srv__CalibConfigWrite_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CalibConfigWrite_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<CalibConfigWrite_Request>) -> bool;
}

// Corresponds to realsense2_camera_msgs__srv__CalibConfigWrite_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CalibConfigWrite_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub calib_config: rosidl_runtime_rs::String,

}



impl Default for CalibConfigWrite_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !realsense2_camera_msgs__srv__CalibConfigWrite_Request__init(&mut msg as *mut _) {
        panic!("Call to realsense2_camera_msgs__srv__CalibConfigWrite_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CalibConfigWrite_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__CalibConfigWrite_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__CalibConfigWrite_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__CalibConfigWrite_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CalibConfigWrite_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CalibConfigWrite_Request where Self: Sized {
  const TYPE_NAME: &'static str = "realsense2_camera_msgs/srv/CalibConfigWrite_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__CalibConfigWrite_Request() }
  }
}


#[link(name = "realsense2_camera_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__CalibConfigWrite_Response() -> *const std::ffi::c_void;
}

#[link(name = "realsense2_camera_msgs__rosidl_generator_c")]
extern "C" {
    fn realsense2_camera_msgs__srv__CalibConfigWrite_Response__init(msg: *mut CalibConfigWrite_Response) -> bool;
    fn realsense2_camera_msgs__srv__CalibConfigWrite_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CalibConfigWrite_Response>, size: usize) -> bool;
    fn realsense2_camera_msgs__srv__CalibConfigWrite_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CalibConfigWrite_Response>);
    fn realsense2_camera_msgs__srv__CalibConfigWrite_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CalibConfigWrite_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<CalibConfigWrite_Response>) -> bool;
}

// Corresponds to realsense2_camera_msgs__srv__CalibConfigWrite_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CalibConfigWrite_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_message: rosidl_runtime_rs::String,

}



impl Default for CalibConfigWrite_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !realsense2_camera_msgs__srv__CalibConfigWrite_Response__init(&mut msg as *mut _) {
        panic!("Call to realsense2_camera_msgs__srv__CalibConfigWrite_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CalibConfigWrite_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__CalibConfigWrite_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__CalibConfigWrite_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__CalibConfigWrite_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CalibConfigWrite_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CalibConfigWrite_Response where Self: Sized {
  const TYPE_NAME: &'static str = "realsense2_camera_msgs/srv/CalibConfigWrite_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__CalibConfigWrite_Response() }
  }
}


#[link(name = "realsense2_camera_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__ApplicationConfigRead_Request() -> *const std::ffi::c_void;
}

#[link(name = "realsense2_camera_msgs__rosidl_generator_c")]
extern "C" {
    fn realsense2_camera_msgs__srv__ApplicationConfigRead_Request__init(msg: *mut ApplicationConfigRead_Request) -> bool;
    fn realsense2_camera_msgs__srv__ApplicationConfigRead_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ApplicationConfigRead_Request>, size: usize) -> bool;
    fn realsense2_camera_msgs__srv__ApplicationConfigRead_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ApplicationConfigRead_Request>);
    fn realsense2_camera_msgs__srv__ApplicationConfigRead_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ApplicationConfigRead_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ApplicationConfigRead_Request>) -> bool;
}

// Corresponds to realsense2_camera_msgs__srv__ApplicationConfigRead_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ApplicationConfigRead_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ApplicationConfigRead_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !realsense2_camera_msgs__srv__ApplicationConfigRead_Request__init(&mut msg as *mut _) {
        panic!("Call to realsense2_camera_msgs__srv__ApplicationConfigRead_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ApplicationConfigRead_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__ApplicationConfigRead_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__ApplicationConfigRead_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__ApplicationConfigRead_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ApplicationConfigRead_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ApplicationConfigRead_Request where Self: Sized {
  const TYPE_NAME: &'static str = "realsense2_camera_msgs/srv/ApplicationConfigRead_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__ApplicationConfigRead_Request() }
  }
}


#[link(name = "realsense2_camera_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__ApplicationConfigRead_Response() -> *const std::ffi::c_void;
}

#[link(name = "realsense2_camera_msgs__rosidl_generator_c")]
extern "C" {
    fn realsense2_camera_msgs__srv__ApplicationConfigRead_Response__init(msg: *mut ApplicationConfigRead_Response) -> bool;
    fn realsense2_camera_msgs__srv__ApplicationConfigRead_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ApplicationConfigRead_Response>, size: usize) -> bool;
    fn realsense2_camera_msgs__srv__ApplicationConfigRead_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ApplicationConfigRead_Response>);
    fn realsense2_camera_msgs__srv__ApplicationConfigRead_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ApplicationConfigRead_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ApplicationConfigRead_Response>) -> bool;
}

// Corresponds to realsense2_camera_msgs__srv__ApplicationConfigRead_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ApplicationConfigRead_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_message: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub application_config: rosidl_runtime_rs::String,

}



impl Default for ApplicationConfigRead_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !realsense2_camera_msgs__srv__ApplicationConfigRead_Response__init(&mut msg as *mut _) {
        panic!("Call to realsense2_camera_msgs__srv__ApplicationConfigRead_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ApplicationConfigRead_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__ApplicationConfigRead_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__ApplicationConfigRead_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__ApplicationConfigRead_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ApplicationConfigRead_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ApplicationConfigRead_Response where Self: Sized {
  const TYPE_NAME: &'static str = "realsense2_camera_msgs/srv/ApplicationConfigRead_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__ApplicationConfigRead_Response() }
  }
}


#[link(name = "realsense2_camera_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__ApplicationConfigWrite_Request() -> *const std::ffi::c_void;
}

#[link(name = "realsense2_camera_msgs__rosidl_generator_c")]
extern "C" {
    fn realsense2_camera_msgs__srv__ApplicationConfigWrite_Request__init(msg: *mut ApplicationConfigWrite_Request) -> bool;
    fn realsense2_camera_msgs__srv__ApplicationConfigWrite_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ApplicationConfigWrite_Request>, size: usize) -> bool;
    fn realsense2_camera_msgs__srv__ApplicationConfigWrite_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ApplicationConfigWrite_Request>);
    fn realsense2_camera_msgs__srv__ApplicationConfigWrite_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ApplicationConfigWrite_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ApplicationConfigWrite_Request>) -> bool;
}

// Corresponds to realsense2_camera_msgs__srv__ApplicationConfigWrite_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ApplicationConfigWrite_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub application_config: rosidl_runtime_rs::String,

}



impl Default for ApplicationConfigWrite_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !realsense2_camera_msgs__srv__ApplicationConfigWrite_Request__init(&mut msg as *mut _) {
        panic!("Call to realsense2_camera_msgs__srv__ApplicationConfigWrite_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ApplicationConfigWrite_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__ApplicationConfigWrite_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__ApplicationConfigWrite_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__ApplicationConfigWrite_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ApplicationConfigWrite_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ApplicationConfigWrite_Request where Self: Sized {
  const TYPE_NAME: &'static str = "realsense2_camera_msgs/srv/ApplicationConfigWrite_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__ApplicationConfigWrite_Request() }
  }
}


#[link(name = "realsense2_camera_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__ApplicationConfigWrite_Response() -> *const std::ffi::c_void;
}

#[link(name = "realsense2_camera_msgs__rosidl_generator_c")]
extern "C" {
    fn realsense2_camera_msgs__srv__ApplicationConfigWrite_Response__init(msg: *mut ApplicationConfigWrite_Response) -> bool;
    fn realsense2_camera_msgs__srv__ApplicationConfigWrite_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ApplicationConfigWrite_Response>, size: usize) -> bool;
    fn realsense2_camera_msgs__srv__ApplicationConfigWrite_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ApplicationConfigWrite_Response>);
    fn realsense2_camera_msgs__srv__ApplicationConfigWrite_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ApplicationConfigWrite_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ApplicationConfigWrite_Response>) -> bool;
}

// Corresponds to realsense2_camera_msgs__srv__ApplicationConfigWrite_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ApplicationConfigWrite_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_message: rosidl_runtime_rs::String,

}



impl Default for ApplicationConfigWrite_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !realsense2_camera_msgs__srv__ApplicationConfigWrite_Response__init(&mut msg as *mut _) {
        panic!("Call to realsense2_camera_msgs__srv__ApplicationConfigWrite_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ApplicationConfigWrite_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__ApplicationConfigWrite_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__ApplicationConfigWrite_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__ApplicationConfigWrite_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ApplicationConfigWrite_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ApplicationConfigWrite_Response where Self: Sized {
  const TYPE_NAME: &'static str = "realsense2_camera_msgs/srv/ApplicationConfigWrite_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__ApplicationConfigWrite_Response() }
  }
}


#[link(name = "realsense2_camera_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__HardwareMonitorCommandSend_Request() -> *const std::ffi::c_void;
}

#[link(name = "realsense2_camera_msgs__rosidl_generator_c")]
extern "C" {
    fn realsense2_camera_msgs__srv__HardwareMonitorCommandSend_Request__init(msg: *mut HardwareMonitorCommandSend_Request) -> bool;
    fn realsense2_camera_msgs__srv__HardwareMonitorCommandSend_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<HardwareMonitorCommandSend_Request>, size: usize) -> bool;
    fn realsense2_camera_msgs__srv__HardwareMonitorCommandSend_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<HardwareMonitorCommandSend_Request>);
    fn realsense2_camera_msgs__srv__HardwareMonitorCommandSend_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<HardwareMonitorCommandSend_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<HardwareMonitorCommandSend_Request>) -> bool;
}

// Corresponds to realsense2_camera_msgs__srv__HardwareMonitorCommandSend_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HardwareMonitorCommandSend_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub opcode: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub param1: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub param2: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub param3: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub param4: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub data: rosidl_runtime_rs::Sequence<u8>,

}



impl Default for HardwareMonitorCommandSend_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !realsense2_camera_msgs__srv__HardwareMonitorCommandSend_Request__init(&mut msg as *mut _) {
        panic!("Call to realsense2_camera_msgs__srv__HardwareMonitorCommandSend_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for HardwareMonitorCommandSend_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__HardwareMonitorCommandSend_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__HardwareMonitorCommandSend_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__HardwareMonitorCommandSend_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for HardwareMonitorCommandSend_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for HardwareMonitorCommandSend_Request where Self: Sized {
  const TYPE_NAME: &'static str = "realsense2_camera_msgs/srv/HardwareMonitorCommandSend_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__HardwareMonitorCommandSend_Request() }
  }
}


#[link(name = "realsense2_camera_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__HardwareMonitorCommandSend_Response() -> *const std::ffi::c_void;
}

#[link(name = "realsense2_camera_msgs__rosidl_generator_c")]
extern "C" {
    fn realsense2_camera_msgs__srv__HardwareMonitorCommandSend_Response__init(msg: *mut HardwareMonitorCommandSend_Response) -> bool;
    fn realsense2_camera_msgs__srv__HardwareMonitorCommandSend_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<HardwareMonitorCommandSend_Response>, size: usize) -> bool;
    fn realsense2_camera_msgs__srv__HardwareMonitorCommandSend_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<HardwareMonitorCommandSend_Response>);
    fn realsense2_camera_msgs__srv__HardwareMonitorCommandSend_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<HardwareMonitorCommandSend_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<HardwareMonitorCommandSend_Response>) -> bool;
}

// Corresponds to realsense2_camera_msgs__srv__HardwareMonitorCommandSend_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HardwareMonitorCommandSend_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: rosidl_runtime_rs::Sequence<u8>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_message: rosidl_runtime_rs::String,

}



impl Default for HardwareMonitorCommandSend_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !realsense2_camera_msgs__srv__HardwareMonitorCommandSend_Response__init(&mut msg as *mut _) {
        panic!("Call to realsense2_camera_msgs__srv__HardwareMonitorCommandSend_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for HardwareMonitorCommandSend_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__HardwareMonitorCommandSend_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__HardwareMonitorCommandSend_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { realsense2_camera_msgs__srv__HardwareMonitorCommandSend_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for HardwareMonitorCommandSend_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for HardwareMonitorCommandSend_Response where Self: Sized {
  const TYPE_NAME: &'static str = "realsense2_camera_msgs/srv/HardwareMonitorCommandSend_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__realsense2_camera_msgs__srv__HardwareMonitorCommandSend_Response() }
  }
}






#[link(name = "realsense2_camera_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__realsense2_camera_msgs__srv__DeviceInfo() -> *const std::ffi::c_void;
}

// Corresponds to realsense2_camera_msgs__srv__DeviceInfo
#[allow(missing_docs, non_camel_case_types)]
pub struct DeviceInfo;

impl rosidl_runtime_rs::Service for DeviceInfo {
    type Request = DeviceInfo_Request;
    type Response = DeviceInfo_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__realsense2_camera_msgs__srv__DeviceInfo() }
    }
}




#[link(name = "realsense2_camera_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__realsense2_camera_msgs__srv__SafetyPresetRead() -> *const std::ffi::c_void;
}

// Corresponds to realsense2_camera_msgs__srv__SafetyPresetRead
#[allow(missing_docs, non_camel_case_types)]
pub struct SafetyPresetRead;

impl rosidl_runtime_rs::Service for SafetyPresetRead {
    type Request = SafetyPresetRead_Request;
    type Response = SafetyPresetRead_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__realsense2_camera_msgs__srv__SafetyPresetRead() }
    }
}




#[link(name = "realsense2_camera_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__realsense2_camera_msgs__srv__SafetyPresetWrite() -> *const std::ffi::c_void;
}

// Corresponds to realsense2_camera_msgs__srv__SafetyPresetWrite
#[allow(missing_docs, non_camel_case_types)]
pub struct SafetyPresetWrite;

impl rosidl_runtime_rs::Service for SafetyPresetWrite {
    type Request = SafetyPresetWrite_Request;
    type Response = SafetyPresetWrite_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__realsense2_camera_msgs__srv__SafetyPresetWrite() }
    }
}




#[link(name = "realsense2_camera_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__realsense2_camera_msgs__srv__SafetyInterfaceConfigRead() -> *const std::ffi::c_void;
}

// Corresponds to realsense2_camera_msgs__srv__SafetyInterfaceConfigRead
#[allow(missing_docs, non_camel_case_types)]
pub struct SafetyInterfaceConfigRead;

impl rosidl_runtime_rs::Service for SafetyInterfaceConfigRead {
    type Request = SafetyInterfaceConfigRead_Request;
    type Response = SafetyInterfaceConfigRead_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__realsense2_camera_msgs__srv__SafetyInterfaceConfigRead() }
    }
}




#[link(name = "realsense2_camera_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__realsense2_camera_msgs__srv__SafetyInterfaceConfigWrite() -> *const std::ffi::c_void;
}

// Corresponds to realsense2_camera_msgs__srv__SafetyInterfaceConfigWrite
#[allow(missing_docs, non_camel_case_types)]
pub struct SafetyInterfaceConfigWrite;

impl rosidl_runtime_rs::Service for SafetyInterfaceConfigWrite {
    type Request = SafetyInterfaceConfigWrite_Request;
    type Response = SafetyInterfaceConfigWrite_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__realsense2_camera_msgs__srv__SafetyInterfaceConfigWrite() }
    }
}




#[link(name = "realsense2_camera_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__realsense2_camera_msgs__srv__CalibConfigRead() -> *const std::ffi::c_void;
}

// Corresponds to realsense2_camera_msgs__srv__CalibConfigRead
#[allow(missing_docs, non_camel_case_types)]
pub struct CalibConfigRead;

impl rosidl_runtime_rs::Service for CalibConfigRead {
    type Request = CalibConfigRead_Request;
    type Response = CalibConfigRead_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__realsense2_camera_msgs__srv__CalibConfigRead() }
    }
}




#[link(name = "realsense2_camera_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__realsense2_camera_msgs__srv__CalibConfigWrite() -> *const std::ffi::c_void;
}

// Corresponds to realsense2_camera_msgs__srv__CalibConfigWrite
#[allow(missing_docs, non_camel_case_types)]
pub struct CalibConfigWrite;

impl rosidl_runtime_rs::Service for CalibConfigWrite {
    type Request = CalibConfigWrite_Request;
    type Response = CalibConfigWrite_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__realsense2_camera_msgs__srv__CalibConfigWrite() }
    }
}




#[link(name = "realsense2_camera_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__realsense2_camera_msgs__srv__ApplicationConfigRead() -> *const std::ffi::c_void;
}

// Corresponds to realsense2_camera_msgs__srv__ApplicationConfigRead
#[allow(missing_docs, non_camel_case_types)]
pub struct ApplicationConfigRead;

impl rosidl_runtime_rs::Service for ApplicationConfigRead {
    type Request = ApplicationConfigRead_Request;
    type Response = ApplicationConfigRead_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__realsense2_camera_msgs__srv__ApplicationConfigRead() }
    }
}




#[link(name = "realsense2_camera_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__realsense2_camera_msgs__srv__ApplicationConfigWrite() -> *const std::ffi::c_void;
}

// Corresponds to realsense2_camera_msgs__srv__ApplicationConfigWrite
#[allow(missing_docs, non_camel_case_types)]
pub struct ApplicationConfigWrite;

impl rosidl_runtime_rs::Service for ApplicationConfigWrite {
    type Request = ApplicationConfigWrite_Request;
    type Response = ApplicationConfigWrite_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__realsense2_camera_msgs__srv__ApplicationConfigWrite() }
    }
}




#[link(name = "realsense2_camera_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__realsense2_camera_msgs__srv__HardwareMonitorCommandSend() -> *const std::ffi::c_void;
}

// Corresponds to realsense2_camera_msgs__srv__HardwareMonitorCommandSend
#[allow(missing_docs, non_camel_case_types)]
pub struct HardwareMonitorCommandSend;

impl rosidl_runtime_rs::Service for HardwareMonitorCommandSend {
    type Request = HardwareMonitorCommandSend_Request;
    type Response = HardwareMonitorCommandSend_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__realsense2_camera_msgs__srv__HardwareMonitorCommandSend() }
    }
}


