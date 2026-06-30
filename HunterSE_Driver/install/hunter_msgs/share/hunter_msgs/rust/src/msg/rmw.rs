#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "hunter_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__hunter_msgs__msg__HunterActuatorState() -> *const std::ffi::c_void;
}

#[link(name = "hunter_msgs__rosidl_generator_c")]
extern "C" {
    fn hunter_msgs__msg__HunterActuatorState__init(msg: *mut HunterActuatorState) -> bool;
    fn hunter_msgs__msg__HunterActuatorState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<HunterActuatorState>, size: usize) -> bool;
    fn hunter_msgs__msg__HunterActuatorState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<HunterActuatorState>);
    fn hunter_msgs__msg__HunterActuatorState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<HunterActuatorState>, out_seq: *mut rosidl_runtime_rs::Sequence<HunterActuatorState>) -> bool;
}

// Corresponds to hunter_msgs__msg__HunterActuatorState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// define DRIVER_STATE_INPUT_VOLTAGE_LOW_MASK ((uint8_t)0x01)
/// define DRIVER_STATE_MOTOR_OVERHEAT_MASK ((uint8_t)0x02)
/// define DRIVER_STATE_DRIVER_OVERLOAD_MASK ((uint8_t)0x04)
/// define DRIVER_STATE_DRIVER_OVERHEAT_MASK ((uint8_t)0x08)
/// define DRIVER_STATE_SENSOR_FAULT_MASK ((uint8_t)0x10)
/// define DRIVER_STATE_DRIVER_FAULT_MASK ((uint8_t)0x20)
/// define DRIVER_STATE_DRIVER_ENABLED_MASK ((uint8_t)0x40)
/// define DRIVER_STATE_DRIVER_RESET_MASK ((uint8_t)0x80)

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HunterActuatorState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub motor_id: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rpm: i16,


    // This member is not documented.
    #[allow(missing_docs)]
    pub current: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub pulse_count: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub driver_voltage: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub driver_temperature: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub motor_temperature: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub driver_state: u8,

}



impl Default for HunterActuatorState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !hunter_msgs__msg__HunterActuatorState__init(&mut msg as *mut _) {
        panic!("Call to hunter_msgs__msg__HunterActuatorState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for HunterActuatorState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { hunter_msgs__msg__HunterActuatorState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { hunter_msgs__msg__HunterActuatorState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { hunter_msgs__msg__HunterActuatorState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for HunterActuatorState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for HunterActuatorState where Self: Sized {
  const TYPE_NAME: &'static str = "hunter_msgs/msg/HunterActuatorState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__hunter_msgs__msg__HunterActuatorState() }
  }
}


#[link(name = "hunter_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__hunter_msgs__msg__HunterLightCmd() -> *const std::ffi::c_void;
}

#[link(name = "hunter_msgs__rosidl_generator_c")]
extern "C" {
    fn hunter_msgs__msg__HunterLightCmd__init(msg: *mut HunterLightCmd) -> bool;
    fn hunter_msgs__msg__HunterLightCmd__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<HunterLightCmd>, size: usize) -> bool;
    fn hunter_msgs__msg__HunterLightCmd__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<HunterLightCmd>);
    fn hunter_msgs__msg__HunterLightCmd__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<HunterLightCmd>, out_seq: *mut rosidl_runtime_rs::Sequence<HunterLightCmd>) -> bool;
}

// Corresponds to hunter_msgs__msg__HunterLightCmd
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HunterLightCmd {

    // This member is not documented.
    #[allow(missing_docs)]
    pub cmd_ctrl_allowed: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub front_mode: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub front_custom_value: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rear_mode: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rear_custom_value: u8,

}

impl HunterLightCmd {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const LIGHT_CONST_OFF: u8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const LIGHT_CONST_ON: u8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const LIGHT_BREATH: u8 = 2;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const LIGHT_CUSTOM: u8 = 3;

}


impl Default for HunterLightCmd {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !hunter_msgs__msg__HunterLightCmd__init(&mut msg as *mut _) {
        panic!("Call to hunter_msgs__msg__HunterLightCmd__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for HunterLightCmd {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { hunter_msgs__msg__HunterLightCmd__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { hunter_msgs__msg__HunterLightCmd__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { hunter_msgs__msg__HunterLightCmd__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for HunterLightCmd {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for HunterLightCmd where Self: Sized {
  const TYPE_NAME: &'static str = "hunter_msgs/msg/HunterLightCmd";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__hunter_msgs__msg__HunterLightCmd() }
  }
}


#[link(name = "hunter_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__hunter_msgs__msg__HunterLightState() -> *const std::ffi::c_void;
}

#[link(name = "hunter_msgs__rosidl_generator_c")]
extern "C" {
    fn hunter_msgs__msg__HunterLightState__init(msg: *mut HunterLightState) -> bool;
    fn hunter_msgs__msg__HunterLightState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<HunterLightState>, size: usize) -> bool;
    fn hunter_msgs__msg__HunterLightState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<HunterLightState>);
    fn hunter_msgs__msg__HunterLightState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<HunterLightState>, out_seq: *mut rosidl_runtime_rs::Sequence<HunterLightState>) -> bool;
}

// Corresponds to hunter_msgs__msg__HunterLightState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HunterLightState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub mode: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub custom_value: u8,

}



impl Default for HunterLightState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !hunter_msgs__msg__HunterLightState__init(&mut msg as *mut _) {
        panic!("Call to hunter_msgs__msg__HunterLightState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for HunterLightState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { hunter_msgs__msg__HunterLightState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { hunter_msgs__msg__HunterLightState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { hunter_msgs__msg__HunterLightState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for HunterLightState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for HunterLightState where Self: Sized {
  const TYPE_NAME: &'static str = "hunter_msgs/msg/HunterLightState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__hunter_msgs__msg__HunterLightState() }
  }
}


#[link(name = "hunter_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__hunter_msgs__msg__HunterRCState() -> *const std::ffi::c_void;
}

#[link(name = "hunter_msgs__rosidl_generator_c")]
extern "C" {
    fn hunter_msgs__msg__HunterRCState__init(msg: *mut HunterRCState) -> bool;
    fn hunter_msgs__msg__HunterRCState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<HunterRCState>, size: usize) -> bool;
    fn hunter_msgs__msg__HunterRCState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<HunterRCState>);
    fn hunter_msgs__msg__HunterRCState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<HunterRCState>, out_seq: *mut rosidl_runtime_rs::Sequence<HunterRCState>) -> bool;
}

// Corresponds to hunter_msgs__msg__HunterRCState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HunterRCState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub swa: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub swb: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub swc: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub swd: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stick_right_v: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stick_right_h: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stick_left_v: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stick_left_h: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub var_a: i8,

}



impl Default for HunterRCState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !hunter_msgs__msg__HunterRCState__init(&mut msg as *mut _) {
        panic!("Call to hunter_msgs__msg__HunterRCState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for HunterRCState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { hunter_msgs__msg__HunterRCState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { hunter_msgs__msg__HunterRCState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { hunter_msgs__msg__HunterRCState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for HunterRCState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for HunterRCState where Self: Sized {
  const TYPE_NAME: &'static str = "hunter_msgs/msg/HunterRCState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__hunter_msgs__msg__HunterRCState() }
  }
}


#[link(name = "hunter_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__hunter_msgs__msg__HunterStatus() -> *const std::ffi::c_void;
}

#[link(name = "hunter_msgs__rosidl_generator_c")]
extern "C" {
    fn hunter_msgs__msg__HunterStatus__init(msg: *mut HunterStatus) -> bool;
    fn hunter_msgs__msg__HunterStatus__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<HunterStatus>, size: usize) -> bool;
    fn hunter_msgs__msg__HunterStatus__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<HunterStatus>);
    fn hunter_msgs__msg__HunterStatus__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<HunterStatus>, out_seq: *mut rosidl_runtime_rs::Sequence<HunterStatus>) -> bool;
}

// Corresponds to hunter_msgs__msg__HunterStatus
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HunterStatus {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,

    /// motion state
    pub linear_velocity: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub steering_angle: f64,

    /// base state
    pub vehicle_state: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub control_mode: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_code: u16,


    // This member is not documented.
    #[allow(missing_docs)]
    pub battery_voltage: f64,

    /// motor state
    pub actuator_states: [super::super::msg::rmw::HunterActuatorState; 3],

}

impl HunterStatus {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const MOTOR_ID_FRONT_RIGHT: i8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const MOTOR_ID_FRONT_LEFT: i8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const MOTOR_ID_REAR_RIGHT: i8 = 2;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const MOTOR_ID_REAR_LEFT: i8 = 3;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const LIGHT_ID_FRONT: i8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const LIGHT_ID_REAR: i8 = 1;

}


impl Default for HunterStatus {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !hunter_msgs__msg__HunterStatus__init(&mut msg as *mut _) {
        panic!("Call to hunter_msgs__msg__HunterStatus__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for HunterStatus {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { hunter_msgs__msg__HunterStatus__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { hunter_msgs__msg__HunterStatus__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { hunter_msgs__msg__HunterStatus__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for HunterStatus {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for HunterStatus where Self: Sized {
  const TYPE_NAME: &'static str = "hunter_msgs/msg/HunterStatus";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__hunter_msgs__msg__HunterStatus() }
  }
}


