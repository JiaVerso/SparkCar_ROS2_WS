#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to realsense2_camera_msgs__srv__DeviceInfo_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DeviceInfo_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for DeviceInfo_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::DeviceInfo_Request::default())
  }
}

impl rosidl_runtime_rs::Message for DeviceInfo_Request {
  type RmwMsg = super::srv::rmw::DeviceInfo_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to realsense2_camera_msgs__srv__DeviceInfo_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DeviceInfo_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub device_name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub serial_number: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub firmware_version: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub usb_type_descriptor: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub firmware_update_id: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub sensors: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub physical_port: std::string::String,

}



impl Default for DeviceInfo_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::DeviceInfo_Response::default())
  }
}

impl rosidl_runtime_rs::Message for DeviceInfo_Response {
  type RmwMsg = super::srv::rmw::DeviceInfo_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        device_name: msg.device_name.as_str().into(),
        serial_number: msg.serial_number.as_str().into(),
        firmware_version: msg.firmware_version.as_str().into(),
        usb_type_descriptor: msg.usb_type_descriptor.as_str().into(),
        firmware_update_id: msg.firmware_update_id.as_str().into(),
        sensors: msg.sensors.as_str().into(),
        physical_port: msg.physical_port.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        device_name: msg.device_name.as_str().into(),
        serial_number: msg.serial_number.as_str().into(),
        firmware_version: msg.firmware_version.as_str().into(),
        usb_type_descriptor: msg.usb_type_descriptor.as_str().into(),
        firmware_update_id: msg.firmware_update_id.as_str().into(),
        sensors: msg.sensors.as_str().into(),
        physical_port: msg.physical_port.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      device_name: msg.device_name.to_string(),
      serial_number: msg.serial_number.to_string(),
      firmware_version: msg.firmware_version.to_string(),
      usb_type_descriptor: msg.usb_type_descriptor.to_string(),
      firmware_update_id: msg.firmware_update_id.to_string(),
      sensors: msg.sensors.to_string(),
      physical_port: msg.physical_port.to_string(),
    }
  }
}


// Corresponds to realsense2_camera_msgs__srv__SafetyPresetRead_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SafetyPresetRead_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: u8,

}



impl Default for SafetyPresetRead_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SafetyPresetRead_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SafetyPresetRead_Request {
  type RmwMsg = super::srv::rmw::SafetyPresetRead_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        index: msg.index,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      index: msg.index,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      index: msg.index,
    }
  }
}


// Corresponds to realsense2_camera_msgs__srv__SafetyPresetRead_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SafetyPresetRead_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_message: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub safety_preset: std::string::String,

}



impl Default for SafetyPresetRead_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SafetyPresetRead_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SafetyPresetRead_Response {
  type RmwMsg = super::srv::rmw::SafetyPresetRead_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        error_message: msg.error_message.as_str().into(),
        safety_preset: msg.safety_preset.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        error_message: msg.error_message.as_str().into(),
        safety_preset: msg.safety_preset.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      error_message: msg.error_message.to_string(),
      safety_preset: msg.safety_preset.to_string(),
    }
  }
}


// Corresponds to realsense2_camera_msgs__srv__SafetyPresetWrite_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SafetyPresetWrite_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub safety_preset: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub index: u8,

}



impl Default for SafetyPresetWrite_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SafetyPresetWrite_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SafetyPresetWrite_Request {
  type RmwMsg = super::srv::rmw::SafetyPresetWrite_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        safety_preset: msg.safety_preset.as_str().into(),
        index: msg.index,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        safety_preset: msg.safety_preset.as_str().into(),
      index: msg.index,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      safety_preset: msg.safety_preset.to_string(),
      index: msg.index,
    }
  }
}


// Corresponds to realsense2_camera_msgs__srv__SafetyPresetWrite_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SafetyPresetWrite_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_message: std::string::String,

}



impl Default for SafetyPresetWrite_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SafetyPresetWrite_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SafetyPresetWrite_Response {
  type RmwMsg = super::srv::rmw::SafetyPresetWrite_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        error_message: msg.error_message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        error_message: msg.error_message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      error_message: msg.error_message.to_string(),
    }
  }
}


// Corresponds to realsense2_camera_msgs__srv__SafetyInterfaceConfigRead_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SafetyInterfaceConfigRead_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub calib_location: u8,

}



impl Default for SafetyInterfaceConfigRead_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SafetyInterfaceConfigRead_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SafetyInterfaceConfigRead_Request {
  type RmwMsg = super::srv::rmw::SafetyInterfaceConfigRead_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        calib_location: msg.calib_location,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      calib_location: msg.calib_location,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      calib_location: msg.calib_location,
    }
  }
}


// Corresponds to realsense2_camera_msgs__srv__SafetyInterfaceConfigRead_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SafetyInterfaceConfigRead_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_message: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub safety_interface_config: std::string::String,

}



impl Default for SafetyInterfaceConfigRead_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SafetyInterfaceConfigRead_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SafetyInterfaceConfigRead_Response {
  type RmwMsg = super::srv::rmw::SafetyInterfaceConfigRead_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        error_message: msg.error_message.as_str().into(),
        safety_interface_config: msg.safety_interface_config.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        error_message: msg.error_message.as_str().into(),
        safety_interface_config: msg.safety_interface_config.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      error_message: msg.error_message.to_string(),
      safety_interface_config: msg.safety_interface_config.to_string(),
    }
  }
}


// Corresponds to realsense2_camera_msgs__srv__SafetyInterfaceConfigWrite_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SafetyInterfaceConfigWrite_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub safety_interface_config: std::string::String,

}



impl Default for SafetyInterfaceConfigWrite_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SafetyInterfaceConfigWrite_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SafetyInterfaceConfigWrite_Request {
  type RmwMsg = super::srv::rmw::SafetyInterfaceConfigWrite_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        safety_interface_config: msg.safety_interface_config.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        safety_interface_config: msg.safety_interface_config.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      safety_interface_config: msg.safety_interface_config.to_string(),
    }
  }
}


// Corresponds to realsense2_camera_msgs__srv__SafetyInterfaceConfigWrite_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SafetyInterfaceConfigWrite_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_message: std::string::String,

}



impl Default for SafetyInterfaceConfigWrite_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SafetyInterfaceConfigWrite_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SafetyInterfaceConfigWrite_Response {
  type RmwMsg = super::srv::rmw::SafetyInterfaceConfigWrite_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        error_message: msg.error_message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        error_message: msg.error_message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      error_message: msg.error_message.to_string(),
    }
  }
}


// Corresponds to realsense2_camera_msgs__srv__CalibConfigRead_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CalibConfigRead_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for CalibConfigRead_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::CalibConfigRead_Request::default())
  }
}

impl rosidl_runtime_rs::Message for CalibConfigRead_Request {
  type RmwMsg = super::srv::rmw::CalibConfigRead_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to realsense2_camera_msgs__srv__CalibConfigRead_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CalibConfigRead_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_message: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub calib_config: std::string::String,

}



impl Default for CalibConfigRead_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::CalibConfigRead_Response::default())
  }
}

impl rosidl_runtime_rs::Message for CalibConfigRead_Response {
  type RmwMsg = super::srv::rmw::CalibConfigRead_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        error_message: msg.error_message.as_str().into(),
        calib_config: msg.calib_config.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        error_message: msg.error_message.as_str().into(),
        calib_config: msg.calib_config.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      error_message: msg.error_message.to_string(),
      calib_config: msg.calib_config.to_string(),
    }
  }
}


// Corresponds to realsense2_camera_msgs__srv__CalibConfigWrite_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CalibConfigWrite_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub calib_config: std::string::String,

}



impl Default for CalibConfigWrite_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::CalibConfigWrite_Request::default())
  }
}

impl rosidl_runtime_rs::Message for CalibConfigWrite_Request {
  type RmwMsg = super::srv::rmw::CalibConfigWrite_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        calib_config: msg.calib_config.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        calib_config: msg.calib_config.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      calib_config: msg.calib_config.to_string(),
    }
  }
}


// Corresponds to realsense2_camera_msgs__srv__CalibConfigWrite_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CalibConfigWrite_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_message: std::string::String,

}



impl Default for CalibConfigWrite_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::CalibConfigWrite_Response::default())
  }
}

impl rosidl_runtime_rs::Message for CalibConfigWrite_Response {
  type RmwMsg = super::srv::rmw::CalibConfigWrite_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        error_message: msg.error_message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        error_message: msg.error_message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      error_message: msg.error_message.to_string(),
    }
  }
}


// Corresponds to realsense2_camera_msgs__srv__ApplicationConfigRead_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ApplicationConfigRead_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ApplicationConfigRead_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ApplicationConfigRead_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ApplicationConfigRead_Request {
  type RmwMsg = super::srv::rmw::ApplicationConfigRead_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to realsense2_camera_msgs__srv__ApplicationConfigRead_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ApplicationConfigRead_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_message: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub application_config: std::string::String,

}



impl Default for ApplicationConfigRead_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ApplicationConfigRead_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ApplicationConfigRead_Response {
  type RmwMsg = super::srv::rmw::ApplicationConfigRead_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        error_message: msg.error_message.as_str().into(),
        application_config: msg.application_config.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        error_message: msg.error_message.as_str().into(),
        application_config: msg.application_config.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      error_message: msg.error_message.to_string(),
      application_config: msg.application_config.to_string(),
    }
  }
}


// Corresponds to realsense2_camera_msgs__srv__ApplicationConfigWrite_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ApplicationConfigWrite_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub application_config: std::string::String,

}



impl Default for ApplicationConfigWrite_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ApplicationConfigWrite_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ApplicationConfigWrite_Request {
  type RmwMsg = super::srv::rmw::ApplicationConfigWrite_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        application_config: msg.application_config.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        application_config: msg.application_config.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      application_config: msg.application_config.to_string(),
    }
  }
}


// Corresponds to realsense2_camera_msgs__srv__ApplicationConfigWrite_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ApplicationConfigWrite_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_message: std::string::String,

}



impl Default for ApplicationConfigWrite_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ApplicationConfigWrite_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ApplicationConfigWrite_Response {
  type RmwMsg = super::srv::rmw::ApplicationConfigWrite_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        error_message: msg.error_message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        error_message: msg.error_message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      error_message: msg.error_message.to_string(),
    }
  }
}


// Corresponds to realsense2_camera_msgs__srv__HardwareMonitorCommandSend_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    pub data: Vec<u8>,

}



impl Default for HardwareMonitorCommandSend_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::HardwareMonitorCommandSend_Request::default())
  }
}

impl rosidl_runtime_rs::Message for HardwareMonitorCommandSend_Request {
  type RmwMsg = super::srv::rmw::HardwareMonitorCommandSend_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        opcode: msg.opcode,
        param1: msg.param1,
        param2: msg.param2,
        param3: msg.param3,
        param4: msg.param4,
        data: msg.data.into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      opcode: msg.opcode,
      param1: msg.param1,
      param2: msg.param2,
      param3: msg.param3,
      param4: msg.param4,
        data: msg.data.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      opcode: msg.opcode,
      param1: msg.param1,
      param2: msg.param2,
      param3: msg.param3,
      param4: msg.param4,
      data: msg.data
          .into_iter()
          .collect(),
    }
  }
}


// Corresponds to realsense2_camera_msgs__srv__HardwareMonitorCommandSend_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HardwareMonitorCommandSend_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: Vec<u8>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_message: std::string::String,

}



impl Default for HardwareMonitorCommandSend_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::HardwareMonitorCommandSend_Response::default())
  }
}

impl rosidl_runtime_rs::Message for HardwareMonitorCommandSend_Response {
  type RmwMsg = super::srv::rmw::HardwareMonitorCommandSend_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        result: msg.result.into(),
        error_message: msg.error_message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        result: msg.result.as_slice().into(),
        error_message: msg.error_message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      result: msg.result
          .into_iter()
          .collect(),
      error_message: msg.error_message.to_string(),
    }
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


