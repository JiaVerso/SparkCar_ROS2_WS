#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to teb_msgs__msg__FeedbackMsg
/// Message that contains intermediate results 
/// and diagnostics of the (predictive) planner.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FeedbackMsg {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,

    /// The planned trajectory (or if multiple plans exist, all of them)
    pub trajectories: Vec<super::msg::TrajectoryMsg>,

    /// Index of the trajectory in 'trajectories' that is selected currently
    pub selected_trajectory_idx: u16,

    /// List of active obstacles
    pub obstacles_msg: costmap_converter_msgs::msg::ObstacleArrayMsg,

}



impl Default for FeedbackMsg {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::FeedbackMsg::default())
  }
}

impl rosidl_runtime_rs::Message for FeedbackMsg {
  type RmwMsg = super::msg::rmw::FeedbackMsg;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        trajectories: msg.trajectories
          .into_iter()
          .map(|elem| super::msg::TrajectoryMsg::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        selected_trajectory_idx: msg.selected_trajectory_idx,
        obstacles_msg: costmap_converter_msgs::msg::ObstacleArrayMsg::into_rmw_message(std::borrow::Cow::Owned(msg.obstacles_msg)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        trajectories: msg.trajectories
          .iter()
          .map(|elem| super::msg::TrajectoryMsg::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      selected_trajectory_idx: msg.selected_trajectory_idx,
        obstacles_msg: costmap_converter_msgs::msg::ObstacleArrayMsg::into_rmw_message(std::borrow::Cow::Borrowed(&msg.obstacles_msg)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      trajectories: msg.trajectories
          .into_iter()
          .map(super::msg::TrajectoryMsg::from_rmw_message)
          .collect(),
      selected_trajectory_idx: msg.selected_trajectory_idx,
      obstacles_msg: costmap_converter_msgs::msg::ObstacleArrayMsg::from_rmw_message(msg.obstacles_msg),
    }
  }
}


// Corresponds to teb_msgs__msg__TrajectoryMsg
/// Message that contains a trajectory for mobile robot navigation

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TrajectoryMsg {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub trajectory: Vec<super::msg::TrajectoryPointMsg>,

}



impl Default for TrajectoryMsg {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::TrajectoryMsg::default())
  }
}

impl rosidl_runtime_rs::Message for TrajectoryMsg {
  type RmwMsg = super::msg::rmw::TrajectoryMsg;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        trajectory: msg.trajectory
          .into_iter()
          .map(|elem| super::msg::TrajectoryPointMsg::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        trajectory: msg.trajectory
          .iter()
          .map(|elem| super::msg::TrajectoryPointMsg::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      trajectory: msg.trajectory
          .into_iter()
          .map(super::msg::TrajectoryPointMsg::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to teb_msgs__msg__TrajectoryPointMsg
/// Message that contains single point on a trajectory suited for mobile navigation.
/// The trajectory is described by a sequence of poses, velocities,
/// accelerations and temporal information.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TrajectoryPointMsg {
    /// Why this message type?
    /// nav_msgs/Path describes only a path without temporal information.
    /// trajectory_msgs package contains only messages for joint trajectories.
    /// The pose of the robot
    pub pose: geometry_msgs::msg::Pose,

    /// Corresponding velocity
    pub velocity: geometry_msgs::msg::Twist,

    /// Corresponding acceleration
    pub acceleration: geometry_msgs::msg::Twist,


    // This member is not documented.
    #[allow(missing_docs)]
    pub time_from_start: builtin_interfaces::msg::Duration,

}



impl Default for TrajectoryPointMsg {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::TrajectoryPointMsg::default())
  }
}

impl rosidl_runtime_rs::Message for TrajectoryPointMsg {
  type RmwMsg = super::msg::rmw::TrajectoryPointMsg;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        pose: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Owned(msg.pose)).into_owned(),
        velocity: geometry_msgs::msg::Twist::into_rmw_message(std::borrow::Cow::Owned(msg.velocity)).into_owned(),
        acceleration: geometry_msgs::msg::Twist::into_rmw_message(std::borrow::Cow::Owned(msg.acceleration)).into_owned(),
        time_from_start: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Owned(msg.time_from_start)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        pose: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Borrowed(&msg.pose)).into_owned(),
        velocity: geometry_msgs::msg::Twist::into_rmw_message(std::borrow::Cow::Borrowed(&msg.velocity)).into_owned(),
        acceleration: geometry_msgs::msg::Twist::into_rmw_message(std::borrow::Cow::Borrowed(&msg.acceleration)).into_owned(),
        time_from_start: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Borrowed(&msg.time_from_start)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      pose: geometry_msgs::msg::Pose::from_rmw_message(msg.pose),
      velocity: geometry_msgs::msg::Twist::from_rmw_message(msg.velocity),
      acceleration: geometry_msgs::msg::Twist::from_rmw_message(msg.acceleration),
      time_from_start: builtin_interfaces::msg::Duration::from_rmw_message(msg.time_from_start),
    }
  }
}


