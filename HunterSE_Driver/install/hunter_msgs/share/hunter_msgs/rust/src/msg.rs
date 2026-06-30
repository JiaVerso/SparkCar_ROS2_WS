#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to hunter_msgs__msg__HunterActuatorState
/// define DRIVER_STATE_INPUT_VOLTAGE_LOW_MASK ((uint8_t)0x01)
/// define DRIVER_STATE_MOTOR_OVERHEAT_MASK ((uint8_t)0x02)
/// define DRIVER_STATE_DRIVER_OVERLOAD_MASK ((uint8_t)0x04)
/// define DRIVER_STATE_DRIVER_OVERHEAT_MASK ((uint8_t)0x08)
/// define DRIVER_STATE_SENSOR_FAULT_MASK ((uint8_t)0x10)
/// define DRIVER_STATE_DRIVER_FAULT_MASK ((uint8_t)0x20)
/// define DRIVER_STATE_DRIVER_ENABLED_MASK ((uint8_t)0x40)
/// define DRIVER_STATE_DRIVER_RESET_MASK ((uint8_t)0x80)

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::HunterActuatorState::default())
  }
}

impl rosidl_runtime_rs::Message for HunterActuatorState {
  type RmwMsg = super::msg::rmw::HunterActuatorState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        motor_id: msg.motor_id,
        rpm: msg.rpm,
        current: msg.current,
        pulse_count: msg.pulse_count,
        driver_voltage: msg.driver_voltage,
        driver_temperature: msg.driver_temperature,
        motor_temperature: msg.motor_temperature,
        driver_state: msg.driver_state,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      motor_id: msg.motor_id,
      rpm: msg.rpm,
      current: msg.current,
      pulse_count: msg.pulse_count,
      driver_voltage: msg.driver_voltage,
      driver_temperature: msg.driver_temperature,
      motor_temperature: msg.motor_temperature,
      driver_state: msg.driver_state,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      motor_id: msg.motor_id,
      rpm: msg.rpm,
      current: msg.current,
      pulse_count: msg.pulse_count,
      driver_voltage: msg.driver_voltage,
      driver_temperature: msg.driver_temperature,
      motor_temperature: msg.motor_temperature,
      driver_state: msg.driver_state,
    }
  }
}


// Corresponds to hunter_msgs__msg__HunterLightCmd

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::HunterLightCmd::default())
  }
}

impl rosidl_runtime_rs::Message for HunterLightCmd {
  type RmwMsg = super::msg::rmw::HunterLightCmd;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        cmd_ctrl_allowed: msg.cmd_ctrl_allowed,
        front_mode: msg.front_mode,
        front_custom_value: msg.front_custom_value,
        rear_mode: msg.rear_mode,
        rear_custom_value: msg.rear_custom_value,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      cmd_ctrl_allowed: msg.cmd_ctrl_allowed,
      front_mode: msg.front_mode,
      front_custom_value: msg.front_custom_value,
      rear_mode: msg.rear_mode,
      rear_custom_value: msg.rear_custom_value,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      cmd_ctrl_allowed: msg.cmd_ctrl_allowed,
      front_mode: msg.front_mode,
      front_custom_value: msg.front_custom_value,
      rear_mode: msg.rear_mode,
      rear_custom_value: msg.rear_custom_value,
    }
  }
}


// Corresponds to hunter_msgs__msg__HunterLightState

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::HunterLightState::default())
  }
}

impl rosidl_runtime_rs::Message for HunterLightState {
  type RmwMsg = super::msg::rmw::HunterLightState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        mode: msg.mode,
        custom_value: msg.custom_value,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      mode: msg.mode,
      custom_value: msg.custom_value,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      mode: msg.mode,
      custom_value: msg.custom_value,
    }
  }
}


// Corresponds to hunter_msgs__msg__HunterRCState

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::HunterRCState::default())
  }
}

impl rosidl_runtime_rs::Message for HunterRCState {
  type RmwMsg = super::msg::rmw::HunterRCState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        swa: msg.swa,
        swb: msg.swb,
        swc: msg.swc,
        swd: msg.swd,
        stick_right_v: msg.stick_right_v,
        stick_right_h: msg.stick_right_h,
        stick_left_v: msg.stick_left_v,
        stick_left_h: msg.stick_left_h,
        var_a: msg.var_a,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      swa: msg.swa,
      swb: msg.swb,
      swc: msg.swc,
      swd: msg.swd,
      stick_right_v: msg.stick_right_v,
      stick_right_h: msg.stick_right_h,
      stick_left_v: msg.stick_left_v,
      stick_left_h: msg.stick_left_h,
      var_a: msg.var_a,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      swa: msg.swa,
      swb: msg.swb,
      swc: msg.swc,
      swd: msg.swd,
      stick_right_v: msg.stick_right_v,
      stick_right_h: msg.stick_right_h,
      stick_left_v: msg.stick_left_v,
      stick_left_h: msg.stick_left_h,
      var_a: msg.var_a,
    }
  }
}


// Corresponds to hunter_msgs__msg__HunterStatus

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HunterStatus {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,

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
    pub actuator_states: [super::msg::HunterActuatorState; 3],

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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::HunterStatus::default())
  }
}

impl rosidl_runtime_rs::Message for HunterStatus {
  type RmwMsg = super::msg::rmw::HunterStatus;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        linear_velocity: msg.linear_velocity,
        steering_angle: msg.steering_angle,
        vehicle_state: msg.vehicle_state,
        control_mode: msg.control_mode,
        error_code: msg.error_code,
        battery_voltage: msg.battery_voltage,
        actuator_states: msg.actuator_states
          .map(|elem| super::msg::HunterActuatorState::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned()),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      linear_velocity: msg.linear_velocity,
      steering_angle: msg.steering_angle,
      vehicle_state: msg.vehicle_state,
      control_mode: msg.control_mode,
      error_code: msg.error_code,
      battery_voltage: msg.battery_voltage,
        actuator_states: msg.actuator_states
          .iter()
          .map(|elem| super::msg::HunterActuatorState::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect::<Vec<_>>()
          .try_into()
          .unwrap(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      linear_velocity: msg.linear_velocity,
      steering_angle: msg.steering_angle,
      vehicle_state: msg.vehicle_state,
      control_mode: msg.control_mode,
      error_code: msg.error_code,
      battery_voltage: msg.battery_voltage,
      actuator_states: msg.actuator_states
        .map(super::msg::HunterActuatorState::from_rmw_message),
    }
  }
}


