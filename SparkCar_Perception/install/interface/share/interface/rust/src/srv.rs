#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to interface__srv__SaveMaps_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SaveMaps_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub file_path: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub save_patches: bool,

}



impl Default for SaveMaps_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SaveMaps_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SaveMaps_Request {
  type RmwMsg = super::srv::rmw::SaveMaps_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        file_path: msg.file_path.as_str().into(),
        save_patches: msg.save_patches,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        file_path: msg.file_path.as_str().into(),
      save_patches: msg.save_patches,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      file_path: msg.file_path.to_string(),
      save_patches: msg.save_patches,
    }
  }
}


// Corresponds to interface__srv__SaveMaps_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SaveMaps_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for SaveMaps_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SaveMaps_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SaveMaps_Response {
  type RmwMsg = super::srv::rmw::SaveMaps_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
    }
  }
}


// Corresponds to interface__srv__Relocalize_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Relocalize_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub pcd_path: std::string::String,


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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::Relocalize_Request::default())
  }
}

impl rosidl_runtime_rs::Message for Relocalize_Request {
  type RmwMsg = super::srv::rmw::Relocalize_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        pcd_path: msg.pcd_path.as_str().into(),
        x: msg.x,
        y: msg.y,
        z: msg.z,
        yaw: msg.yaw,
        pitch: msg.pitch,
        roll: msg.roll,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        pcd_path: msg.pcd_path.as_str().into(),
      x: msg.x,
      y: msg.y,
      z: msg.z,
      yaw: msg.yaw,
      pitch: msg.pitch,
      roll: msg.roll,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      pcd_path: msg.pcd_path.to_string(),
      x: msg.x,
      y: msg.y,
      z: msg.z,
      yaw: msg.yaw,
      pitch: msg.pitch,
      roll: msg.roll,
    }
  }
}


// Corresponds to interface__srv__Relocalize_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Relocalize_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for Relocalize_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::Relocalize_Response::default())
  }
}

impl rosidl_runtime_rs::Message for Relocalize_Response {
  type RmwMsg = super::srv::rmw::Relocalize_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
    }
  }
}


// Corresponds to interface__srv__IsValid_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct IsValid_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub code: i32,

}



impl Default for IsValid_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::IsValid_Request::default())
  }
}

impl rosidl_runtime_rs::Message for IsValid_Request {
  type RmwMsg = super::srv::rmw::IsValid_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        code: msg.code,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      code: msg.code,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      code: msg.code,
    }
  }
}


// Corresponds to interface__srv__IsValid_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct IsValid_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub valid: bool,

}



impl Default for IsValid_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::IsValid_Response::default())
  }
}

impl rosidl_runtime_rs::Message for IsValid_Response {
  type RmwMsg = super::srv::rmw::IsValid_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        valid: msg.valid,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      valid: msg.valid,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      valid: msg.valid,
    }
  }
}


// Corresponds to interface__srv__RefineMap_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RefineMap_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub maps_path: std::string::String,

}



impl Default for RefineMap_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::RefineMap_Request::default())
  }
}

impl rosidl_runtime_rs::Message for RefineMap_Request {
  type RmwMsg = super::srv::rmw::RefineMap_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        maps_path: msg.maps_path.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        maps_path: msg.maps_path.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      maps_path: msg.maps_path.to_string(),
    }
  }
}


// Corresponds to interface__srv__RefineMap_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RefineMap_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for RefineMap_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::RefineMap_Response::default())
  }
}

impl rosidl_runtime_rs::Message for RefineMap_Response {
  type RmwMsg = super::srv::rmw::RefineMap_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
    }
  }
}


// Corresponds to interface__srv__SavePoses_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SavePoses_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub file_path: std::string::String,

}



impl Default for SavePoses_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SavePoses_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SavePoses_Request {
  type RmwMsg = super::srv::rmw::SavePoses_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        file_path: msg.file_path.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        file_path: msg.file_path.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      file_path: msg.file_path.to_string(),
    }
  }
}


// Corresponds to interface__srv__SavePoses_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SavePoses_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for SavePoses_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SavePoses_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SavePoses_Response {
  type RmwMsg = super::srv::rmw::SavePoses_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
    }
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


