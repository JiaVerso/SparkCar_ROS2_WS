
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to realsense2_camera_msgs__action__TriggeredCalibration_Goal

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TriggeredCalibration_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub json: std::string::String,

}



impl Default for TriggeredCalibration_Goal {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::TriggeredCalibration_Goal::default())
  }
}

impl rosidl_runtime_rs::Message for TriggeredCalibration_Goal {
  type RmwMsg = super::action::rmw::TriggeredCalibration_Goal;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        json: msg.json.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        json: msg.json.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      json: msg.json.to_string(),
    }
  }
}


// Corresponds to realsense2_camera_msgs__action__TriggeredCalibration_Result

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TriggeredCalibration_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_msg: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub calibration: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub health: f32,

}



impl Default for TriggeredCalibration_Result {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::TriggeredCalibration_Result::default())
  }
}

impl rosidl_runtime_rs::Message for TriggeredCalibration_Result {
  type RmwMsg = super::action::rmw::TriggeredCalibration_Result;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        error_msg: msg.error_msg.as_str().into(),
        calibration: msg.calibration.as_str().into(),
        health: msg.health,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        error_msg: msg.error_msg.as_str().into(),
        calibration: msg.calibration.as_str().into(),
      health: msg.health,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      error_msg: msg.error_msg.to_string(),
      calibration: msg.calibration.to_string(),
      health: msg.health,
    }
  }
}


// Corresponds to realsense2_camera_msgs__action__TriggeredCalibration_Feedback

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TriggeredCalibration_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub progress: f32,

}



impl Default for TriggeredCalibration_Feedback {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::TriggeredCalibration_Feedback::default())
  }
}

impl rosidl_runtime_rs::Message for TriggeredCalibration_Feedback {
  type RmwMsg = super::action::rmw::TriggeredCalibration_Feedback;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        progress: msg.progress,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      progress: msg.progress,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      progress: msg.progress,
    }
  }
}


// Corresponds to realsense2_camera_msgs__action__TriggeredCalibration_FeedbackMessage

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TriggeredCalibration_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::action::TriggeredCalibration_Feedback,

}



impl Default for TriggeredCalibration_FeedbackMessage {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::TriggeredCalibration_FeedbackMessage::default())
  }
}

impl rosidl_runtime_rs::Message for TriggeredCalibration_FeedbackMessage {
  type RmwMsg = super::action::rmw::TriggeredCalibration_FeedbackMessage;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        feedback: super::action::TriggeredCalibration_Feedback::into_rmw_message(std::borrow::Cow::Owned(msg.feedback)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        feedback: super::action::TriggeredCalibration_Feedback::into_rmw_message(std::borrow::Cow::Borrowed(&msg.feedback)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      feedback: super::action::TriggeredCalibration_Feedback::from_rmw_message(msg.feedback),
    }
  }
}






// Corresponds to realsense2_camera_msgs__action__TriggeredCalibration_SendGoal_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TriggeredCalibration_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::action::TriggeredCalibration_Goal,

}



impl Default for TriggeredCalibration_SendGoal_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::TriggeredCalibration_SendGoal_Request::default())
  }
}

impl rosidl_runtime_rs::Message for TriggeredCalibration_SendGoal_Request {
  type RmwMsg = super::action::rmw::TriggeredCalibration_SendGoal_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        goal: super::action::TriggeredCalibration_Goal::into_rmw_message(std::borrow::Cow::Owned(msg.goal)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        goal: super::action::TriggeredCalibration_Goal::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      goal: super::action::TriggeredCalibration_Goal::from_rmw_message(msg.goal),
    }
  }
}


// Corresponds to realsense2_camera_msgs__action__TriggeredCalibration_SendGoal_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TriggeredCalibration_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::Time,

}



impl Default for TriggeredCalibration_SendGoal_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::TriggeredCalibration_SendGoal_Response::default())
  }
}

impl rosidl_runtime_rs::Message for TriggeredCalibration_SendGoal_Response {
  type RmwMsg = super::action::rmw::TriggeredCalibration_SendGoal_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.stamp)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.stamp)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      accepted: msg.accepted,
      stamp: builtin_interfaces::msg::Time::from_rmw_message(msg.stamp),
    }
  }
}


// Corresponds to realsense2_camera_msgs__action__TriggeredCalibration_GetResult_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TriggeredCalibration_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,

}



impl Default for TriggeredCalibration_GetResult_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::TriggeredCalibration_GetResult_Request::default())
  }
}

impl rosidl_runtime_rs::Message for TriggeredCalibration_GetResult_Request {
  type RmwMsg = super::action::rmw::TriggeredCalibration_GetResult_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
    }
  }
}


// Corresponds to realsense2_camera_msgs__action__TriggeredCalibration_GetResult_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TriggeredCalibration_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::action::TriggeredCalibration_Result,

}



impl Default for TriggeredCalibration_GetResult_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::TriggeredCalibration_GetResult_Response::default())
  }
}

impl rosidl_runtime_rs::Message for TriggeredCalibration_GetResult_Response {
  type RmwMsg = super::action::rmw::TriggeredCalibration_GetResult_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: msg.status,
        result: super::action::TriggeredCalibration_Result::into_rmw_message(std::borrow::Cow::Owned(msg.result)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      status: msg.status,
        result: super::action::TriggeredCalibration_Result::into_rmw_message(std::borrow::Cow::Borrowed(&msg.result)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: msg.status,
      result: super::action::TriggeredCalibration_Result::from_rmw_message(msg.result),
    }
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






#[link(name = "realsense2_camera_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_action_type_support_handle__realsense2_camera_msgs__action__TriggeredCalibration() -> *const std::ffi::c_void;
}

// Corresponds to realsense2_camera_msgs__action__TriggeredCalibration
#[allow(missing_docs, non_camel_case_types)]
pub struct TriggeredCalibration;

impl rosidl_runtime_rs::Action for TriggeredCalibration {
  // --- Associated types for client library users ---
  /// The goal message defined in the action definition.
  type Goal = TriggeredCalibration_Goal;

  /// The result message defined in the action definition.
  type Result = TriggeredCalibration_Result;

  /// The feedback message defined in the action definition.
  type Feedback = TriggeredCalibration_Feedback;

  // --- Associated types for client library implementation ---
  /// The feedback message with generic fields which wraps the feedback message.
  type FeedbackMessage = super::action::TriggeredCalibration_FeedbackMessage;

  /// The send_goal service using a wrapped version of the goal message as a request.
  type SendGoalService = super::action::TriggeredCalibration_SendGoal;

  /// The generic service to cancel a goal.
  type CancelGoalService = action_msgs::srv::rmw::CancelGoal;

  /// The get_result service using a wrapped version of the result message as a response.
  type GetResultService = super::action::TriggeredCalibration_GetResult;

  // --- Methods for client library implementation ---
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_action_type_support_handle__realsense2_camera_msgs__action__TriggeredCalibration() }
  }

  fn create_goal_request(
    goal_id: &[u8; 16],
    goal: super::action::rmw::TriggeredCalibration_Goal,
  ) -> super::action::rmw::TriggeredCalibration_SendGoal_Request {
   super::action::rmw::TriggeredCalibration_SendGoal_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
      goal,
    }
  }

  fn split_goal_request(
    request: super::action::rmw::TriggeredCalibration_SendGoal_Request,
  ) -> (
    [u8; 16],
   super::action::rmw::TriggeredCalibration_Goal,
  ) {
    (request.goal_id.uuid, request.goal)
  }

  fn create_goal_response(
    accepted: bool,
    stamp: (i32, u32),
  ) -> super::action::rmw::TriggeredCalibration_SendGoal_Response {
   super::action::rmw::TriggeredCalibration_SendGoal_Response {
      accepted,
      stamp: builtin_interfaces::msg::rmw::Time {
        sec: stamp.0,
        nanosec: stamp.1,
      },
    }
  }

  fn get_goal_response_accepted(
    response: &super::action::rmw::TriggeredCalibration_SendGoal_Response,
  ) -> bool {
    response.accepted
  }

  fn get_goal_response_stamp(
    response: &super::action::rmw::TriggeredCalibration_SendGoal_Response,
  ) -> (i32, u32) {
    (response.stamp.sec, response.stamp.nanosec)
  }

  fn create_feedback_message(
    goal_id: &[u8; 16],
    feedback: super::action::rmw::TriggeredCalibration_Feedback,
  ) -> super::action::rmw::TriggeredCalibration_FeedbackMessage {
    let mut message = super::action::rmw::TriggeredCalibration_FeedbackMessage::default();
    message.goal_id.uuid = *goal_id;
    message.feedback = feedback;
    message
  }

  fn split_feedback_message(
    feedback: super::action::rmw::TriggeredCalibration_FeedbackMessage,
  ) -> (
    [u8; 16],
   super::action::rmw::TriggeredCalibration_Feedback,
  ) {
    (feedback.goal_id.uuid, feedback.feedback)
  }

  fn create_result_request(
    goal_id: &[u8; 16],
  ) -> super::action::rmw::TriggeredCalibration_GetResult_Request {
   super::action::rmw::TriggeredCalibration_GetResult_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
    }
  }

  fn get_result_request_uuid(
    request: &super::action::rmw::TriggeredCalibration_GetResult_Request,
  ) -> &[u8; 16] {
    &request.goal_id.uuid
  }

  fn create_result_response(
    status: i8,
    result: super::action::rmw::TriggeredCalibration_Result,
  ) -> super::action::rmw::TriggeredCalibration_GetResult_Response {
   super::action::rmw::TriggeredCalibration_GetResult_Response {
      status,
      result,
    }
  }

  fn split_result_response(
    response: super::action::rmw::TriggeredCalibration_GetResult_Response
  ) -> (
    i8,
   super::action::rmw::TriggeredCalibration_Result,
  ) {
    (response.status, response.result)
  }
}


