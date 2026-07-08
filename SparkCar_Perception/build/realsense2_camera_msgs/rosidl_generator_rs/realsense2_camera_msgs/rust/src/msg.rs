#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to realsense2_camera_msgs__msg__IMUInfo
/// header.frame_id is either set to "imu_accel" or "imu_gyro"
/// to distinguish between "accel" and "gyro" info.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct IMUInfo {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub data: [f64; 12],


    // This member is not documented.
    #[allow(missing_docs)]
    pub noise_variances: [f64; 3],


    // This member is not documented.
    #[allow(missing_docs)]
    pub bias_variances: [f64; 3],

}



impl Default for IMUInfo {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::IMUInfo::default())
  }
}

impl rosidl_runtime_rs::Message for IMUInfo {
  type RmwMsg = super::msg::rmw::IMUInfo;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        data: msg.data,
        noise_variances: msg.noise_variances,
        bias_variances: msg.bias_variances,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        data: msg.data,
        noise_variances: msg.noise_variances,
        bias_variances: msg.bias_variances,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      data: msg.data,
      noise_variances: msg.noise_variances,
      bias_variances: msg.bias_variances,
    }
  }
}


// Corresponds to realsense2_camera_msgs__msg__Extrinsics
/// Cross-stream extrinsics: encodes the topology describing how the different devices are oriented

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Extrinsics {
    /// Column - major 3x3 rotation matrix
    pub rotation: [f64; 9],

    /// Three-element translation vector, in meters
    pub translation: [f64; 3],

}



impl Default for Extrinsics {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Extrinsics::default())
  }
}

impl rosidl_runtime_rs::Message for Extrinsics {
  type RmwMsg = super::msg::rmw::Extrinsics;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        rotation: msg.rotation,
        translation: msg.translation,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        rotation: msg.rotation,
        translation: msg.translation,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      rotation: msg.rotation,
      translation: msg.translation,
    }
  }
}


// Corresponds to realsense2_camera_msgs__msg__Metadata

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Metadata {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub json_data: std::string::String,

}



impl Default for Metadata {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Metadata::default())
  }
}

impl rosidl_runtime_rs::Message for Metadata {
  type RmwMsg = super::msg::rmw::Metadata;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        json_data: msg.json_data.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        json_data: msg.json_data.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      json_data: msg.json_data.to_string(),
    }
  }
}


// Corresponds to realsense2_camera_msgs__msg__RGBD
/// RGBD Message

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RGBD {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rgb_camera_info: sensor_msgs::msg::CameraInfo,


    // This member is not documented.
    #[allow(missing_docs)]
    pub depth_camera_info: sensor_msgs::msg::CameraInfo,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rgb: sensor_msgs::msg::Image,


    // This member is not documented.
    #[allow(missing_docs)]
    pub depth: sensor_msgs::msg::Image,

}



impl Default for RGBD {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::RGBD::default())
  }
}

impl rosidl_runtime_rs::Message for RGBD {
  type RmwMsg = super::msg::rmw::RGBD;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        rgb_camera_info: sensor_msgs::msg::CameraInfo::into_rmw_message(std::borrow::Cow::Owned(msg.rgb_camera_info)).into_owned(),
        depth_camera_info: sensor_msgs::msg::CameraInfo::into_rmw_message(std::borrow::Cow::Owned(msg.depth_camera_info)).into_owned(),
        rgb: sensor_msgs::msg::Image::into_rmw_message(std::borrow::Cow::Owned(msg.rgb)).into_owned(),
        depth: sensor_msgs::msg::Image::into_rmw_message(std::borrow::Cow::Owned(msg.depth)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        rgb_camera_info: sensor_msgs::msg::CameraInfo::into_rmw_message(std::borrow::Cow::Borrowed(&msg.rgb_camera_info)).into_owned(),
        depth_camera_info: sensor_msgs::msg::CameraInfo::into_rmw_message(std::borrow::Cow::Borrowed(&msg.depth_camera_info)).into_owned(),
        rgb: sensor_msgs::msg::Image::into_rmw_message(std::borrow::Cow::Borrowed(&msg.rgb)).into_owned(),
        depth: sensor_msgs::msg::Image::into_rmw_message(std::borrow::Cow::Borrowed(&msg.depth)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      rgb_camera_info: sensor_msgs::msg::CameraInfo::from_rmw_message(msg.rgb_camera_info),
      depth_camera_info: sensor_msgs::msg::CameraInfo::from_rmw_message(msg.depth_camera_info),
      rgb: sensor_msgs::msg::Image::from_rmw_message(msg.rgb),
      depth: sensor_msgs::msg::Image::from_rmw_message(msg.depth),
    }
  }
}


