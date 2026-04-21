#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__interface__srv__SaveMaps_Request() -> *const std::ffi::c_void;
}

#[link(name = "interface__rosidl_generator_c")]
extern "C" {
    fn interface__srv__SaveMaps_Request__init(msg: *mut SaveMaps_Request) -> bool;
    fn interface__srv__SaveMaps_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SaveMaps_Request>, size: usize) -> bool;
    fn interface__srv__SaveMaps_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SaveMaps_Request>);
    fn interface__srv__SaveMaps_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SaveMaps_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SaveMaps_Request>) -> bool;
}

// Corresponds to interface__srv__SaveMaps_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SaveMaps_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub file_path: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub save_patches: bool,

}



impl Default for SaveMaps_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !interface__srv__SaveMaps_Request__init(&mut msg as *mut _) {
        panic!("Call to interface__srv__SaveMaps_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SaveMaps_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { interface__srv__SaveMaps_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { interface__srv__SaveMaps_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { interface__srv__SaveMaps_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SaveMaps_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SaveMaps_Request where Self: Sized {
  const TYPE_NAME: &'static str = "interface/srv/SaveMaps_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__interface__srv__SaveMaps_Request() }
  }
}


#[link(name = "interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__interface__srv__SaveMaps_Response() -> *const std::ffi::c_void;
}

#[link(name = "interface__rosidl_generator_c")]
extern "C" {
    fn interface__srv__SaveMaps_Response__init(msg: *mut SaveMaps_Response) -> bool;
    fn interface__srv__SaveMaps_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SaveMaps_Response>, size: usize) -> bool;
    fn interface__srv__SaveMaps_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SaveMaps_Response>);
    fn interface__srv__SaveMaps_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SaveMaps_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SaveMaps_Response>) -> bool;
}

// Corresponds to interface__srv__SaveMaps_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SaveMaps_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for SaveMaps_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !interface__srv__SaveMaps_Response__init(&mut msg as *mut _) {
        panic!("Call to interface__srv__SaveMaps_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SaveMaps_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { interface__srv__SaveMaps_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { interface__srv__SaveMaps_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { interface__srv__SaveMaps_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SaveMaps_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SaveMaps_Response where Self: Sized {
  const TYPE_NAME: &'static str = "interface/srv/SaveMaps_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__interface__srv__SaveMaps_Response() }
  }
}


#[link(name = "interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__interface__srv__Relocalize_Request() -> *const std::ffi::c_void;
}

#[link(name = "interface__rosidl_generator_c")]
extern "C" {
    fn interface__srv__Relocalize_Request__init(msg: *mut Relocalize_Request) -> bool;
    fn interface__srv__Relocalize_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Relocalize_Request>, size: usize) -> bool;
    fn interface__srv__Relocalize_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Relocalize_Request>);
    fn interface__srv__Relocalize_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Relocalize_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Relocalize_Request>) -> bool;
}

// Corresponds to interface__srv__Relocalize_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Relocalize_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub pcd_path: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub x: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub y: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub z: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub yaw: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub pitch: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub roll: f32,

}



impl Default for Relocalize_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !interface__srv__Relocalize_Request__init(&mut msg as *mut _) {
        panic!("Call to interface__srv__Relocalize_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Relocalize_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { interface__srv__Relocalize_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { interface__srv__Relocalize_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { interface__srv__Relocalize_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Relocalize_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Relocalize_Request where Self: Sized {
  const TYPE_NAME: &'static str = "interface/srv/Relocalize_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__interface__srv__Relocalize_Request() }
  }
}


#[link(name = "interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__interface__srv__Relocalize_Response() -> *const std::ffi::c_void;
}

#[link(name = "interface__rosidl_generator_c")]
extern "C" {
    fn interface__srv__Relocalize_Response__init(msg: *mut Relocalize_Response) -> bool;
    fn interface__srv__Relocalize_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Relocalize_Response>, size: usize) -> bool;
    fn interface__srv__Relocalize_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Relocalize_Response>);
    fn interface__srv__Relocalize_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Relocalize_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Relocalize_Response>) -> bool;
}

// Corresponds to interface__srv__Relocalize_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Relocalize_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for Relocalize_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !interface__srv__Relocalize_Response__init(&mut msg as *mut _) {
        panic!("Call to interface__srv__Relocalize_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Relocalize_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { interface__srv__Relocalize_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { interface__srv__Relocalize_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { interface__srv__Relocalize_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Relocalize_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Relocalize_Response where Self: Sized {
  const TYPE_NAME: &'static str = "interface/srv/Relocalize_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__interface__srv__Relocalize_Response() }
  }
}


#[link(name = "interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__interface__srv__IsValid_Request() -> *const std::ffi::c_void;
}

#[link(name = "interface__rosidl_generator_c")]
extern "C" {
    fn interface__srv__IsValid_Request__init(msg: *mut IsValid_Request) -> bool;
    fn interface__srv__IsValid_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<IsValid_Request>, size: usize) -> bool;
    fn interface__srv__IsValid_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<IsValid_Request>);
    fn interface__srv__IsValid_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<IsValid_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<IsValid_Request>) -> bool;
}

// Corresponds to interface__srv__IsValid_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct IsValid_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub code: i32,

}



impl Default for IsValid_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !interface__srv__IsValid_Request__init(&mut msg as *mut _) {
        panic!("Call to interface__srv__IsValid_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for IsValid_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { interface__srv__IsValid_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { interface__srv__IsValid_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { interface__srv__IsValid_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for IsValid_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for IsValid_Request where Self: Sized {
  const TYPE_NAME: &'static str = "interface/srv/IsValid_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__interface__srv__IsValid_Request() }
  }
}


#[link(name = "interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__interface__srv__IsValid_Response() -> *const std::ffi::c_void;
}

#[link(name = "interface__rosidl_generator_c")]
extern "C" {
    fn interface__srv__IsValid_Response__init(msg: *mut IsValid_Response) -> bool;
    fn interface__srv__IsValid_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<IsValid_Response>, size: usize) -> bool;
    fn interface__srv__IsValid_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<IsValid_Response>);
    fn interface__srv__IsValid_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<IsValid_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<IsValid_Response>) -> bool;
}

// Corresponds to interface__srv__IsValid_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct IsValid_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub valid: bool,

}



impl Default for IsValid_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !interface__srv__IsValid_Response__init(&mut msg as *mut _) {
        panic!("Call to interface__srv__IsValid_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for IsValid_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { interface__srv__IsValid_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { interface__srv__IsValid_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { interface__srv__IsValid_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for IsValid_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for IsValid_Response where Self: Sized {
  const TYPE_NAME: &'static str = "interface/srv/IsValid_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__interface__srv__IsValid_Response() }
  }
}


#[link(name = "interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__interface__srv__RefineMap_Request() -> *const std::ffi::c_void;
}

#[link(name = "interface__rosidl_generator_c")]
extern "C" {
    fn interface__srv__RefineMap_Request__init(msg: *mut RefineMap_Request) -> bool;
    fn interface__srv__RefineMap_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RefineMap_Request>, size: usize) -> bool;
    fn interface__srv__RefineMap_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RefineMap_Request>);
    fn interface__srv__RefineMap_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RefineMap_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<RefineMap_Request>) -> bool;
}

// Corresponds to interface__srv__RefineMap_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RefineMap_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub maps_path: rosidl_runtime_rs::String,

}



impl Default for RefineMap_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !interface__srv__RefineMap_Request__init(&mut msg as *mut _) {
        panic!("Call to interface__srv__RefineMap_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RefineMap_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { interface__srv__RefineMap_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { interface__srv__RefineMap_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { interface__srv__RefineMap_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RefineMap_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RefineMap_Request where Self: Sized {
  const TYPE_NAME: &'static str = "interface/srv/RefineMap_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__interface__srv__RefineMap_Request() }
  }
}


#[link(name = "interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__interface__srv__RefineMap_Response() -> *const std::ffi::c_void;
}

#[link(name = "interface__rosidl_generator_c")]
extern "C" {
    fn interface__srv__RefineMap_Response__init(msg: *mut RefineMap_Response) -> bool;
    fn interface__srv__RefineMap_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RefineMap_Response>, size: usize) -> bool;
    fn interface__srv__RefineMap_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RefineMap_Response>);
    fn interface__srv__RefineMap_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RefineMap_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<RefineMap_Response>) -> bool;
}

// Corresponds to interface__srv__RefineMap_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RefineMap_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for RefineMap_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !interface__srv__RefineMap_Response__init(&mut msg as *mut _) {
        panic!("Call to interface__srv__RefineMap_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RefineMap_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { interface__srv__RefineMap_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { interface__srv__RefineMap_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { interface__srv__RefineMap_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RefineMap_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RefineMap_Response where Self: Sized {
  const TYPE_NAME: &'static str = "interface/srv/RefineMap_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__interface__srv__RefineMap_Response() }
  }
}


#[link(name = "interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__interface__srv__SavePoses_Request() -> *const std::ffi::c_void;
}

#[link(name = "interface__rosidl_generator_c")]
extern "C" {
    fn interface__srv__SavePoses_Request__init(msg: *mut SavePoses_Request) -> bool;
    fn interface__srv__SavePoses_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SavePoses_Request>, size: usize) -> bool;
    fn interface__srv__SavePoses_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SavePoses_Request>);
    fn interface__srv__SavePoses_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SavePoses_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SavePoses_Request>) -> bool;
}

// Corresponds to interface__srv__SavePoses_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SavePoses_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub file_path: rosidl_runtime_rs::String,

}



impl Default for SavePoses_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !interface__srv__SavePoses_Request__init(&mut msg as *mut _) {
        panic!("Call to interface__srv__SavePoses_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SavePoses_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { interface__srv__SavePoses_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { interface__srv__SavePoses_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { interface__srv__SavePoses_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SavePoses_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SavePoses_Request where Self: Sized {
  const TYPE_NAME: &'static str = "interface/srv/SavePoses_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__interface__srv__SavePoses_Request() }
  }
}


#[link(name = "interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__interface__srv__SavePoses_Response() -> *const std::ffi::c_void;
}

#[link(name = "interface__rosidl_generator_c")]
extern "C" {
    fn interface__srv__SavePoses_Response__init(msg: *mut SavePoses_Response) -> bool;
    fn interface__srv__SavePoses_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SavePoses_Response>, size: usize) -> bool;
    fn interface__srv__SavePoses_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SavePoses_Response>);
    fn interface__srv__SavePoses_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SavePoses_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SavePoses_Response>) -> bool;
}

// Corresponds to interface__srv__SavePoses_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SavePoses_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for SavePoses_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !interface__srv__SavePoses_Response__init(&mut msg as *mut _) {
        panic!("Call to interface__srv__SavePoses_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SavePoses_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { interface__srv__SavePoses_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { interface__srv__SavePoses_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { interface__srv__SavePoses_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SavePoses_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SavePoses_Response where Self: Sized {
  const TYPE_NAME: &'static str = "interface/srv/SavePoses_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__interface__srv__SavePoses_Response() }
  }
}






#[link(name = "interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__interface__srv__SaveMaps() -> *const std::ffi::c_void;
}

// Corresponds to interface__srv__SaveMaps
#[allow(missing_docs, non_camel_case_types)]
pub struct SaveMaps;

impl rosidl_runtime_rs::Service for SaveMaps {
    type Request = SaveMaps_Request;
    type Response = SaveMaps_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__interface__srv__SaveMaps() }
    }
}




#[link(name = "interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__interface__srv__Relocalize() -> *const std::ffi::c_void;
}

// Corresponds to interface__srv__Relocalize
#[allow(missing_docs, non_camel_case_types)]
pub struct Relocalize;

impl rosidl_runtime_rs::Service for Relocalize {
    type Request = Relocalize_Request;
    type Response = Relocalize_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__interface__srv__Relocalize() }
    }
}




#[link(name = "interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__interface__srv__IsValid() -> *const std::ffi::c_void;
}

// Corresponds to interface__srv__IsValid
#[allow(missing_docs, non_camel_case_types)]
pub struct IsValid;

impl rosidl_runtime_rs::Service for IsValid {
    type Request = IsValid_Request;
    type Response = IsValid_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__interface__srv__IsValid() }
    }
}




#[link(name = "interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__interface__srv__RefineMap() -> *const std::ffi::c_void;
}

// Corresponds to interface__srv__RefineMap
#[allow(missing_docs, non_camel_case_types)]
pub struct RefineMap;

impl rosidl_runtime_rs::Service for RefineMap {
    type Request = RefineMap_Request;
    type Response = RefineMap_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__interface__srv__RefineMap() }
    }
}




#[link(name = "interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__interface__srv__SavePoses() -> *const std::ffi::c_void;
}

// Corresponds to interface__srv__SavePoses
#[allow(missing_docs, non_camel_case_types)]
pub struct SavePoses;

impl rosidl_runtime_rs::Service for SavePoses {
    type Request = SavePoses_Request;
    type Response = SavePoses_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__interface__srv__SavePoses() }
    }
}


