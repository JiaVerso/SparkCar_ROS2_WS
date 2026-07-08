#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to costmap_converter_msgs__msg__ObstacleMsg
/// Special types:
/// Polygon with 1 vertex: Point obstacle (you might also specify a non-zero value for radius)
/// Polygon with 2 vertices: Line obstacle
/// Polygon with more than 2 vertices: First and last points are assumed to be connected

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ObstacleMsg {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,

    /// Obstacle footprint (polygon descriptions)
    pub polygon: geometry_msgs::msg::Polygon,

    /// Specify the radius for circular/point obstacles
    pub radius: f64,

    /// Obstacle ID
    /// Specify IDs in order to provide (temporal) relationships
    /// between obstacles among multiple messages.
    pub id: i64,

    /// Individual orientation (centroid)
    pub orientation: geometry_msgs::msg::Quaternion,

    /// Individual velocities (centroid)
    pub velocities: geometry_msgs::msg::TwistWithCovariance,

}



impl Default for ObstacleMsg {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ObstacleMsg::default())
  }
}

impl rosidl_runtime_rs::Message for ObstacleMsg {
  type RmwMsg = super::msg::rmw::ObstacleMsg;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        polygon: geometry_msgs::msg::Polygon::into_rmw_message(std::borrow::Cow::Owned(msg.polygon)).into_owned(),
        radius: msg.radius,
        id: msg.id,
        orientation: geometry_msgs::msg::Quaternion::into_rmw_message(std::borrow::Cow::Owned(msg.orientation)).into_owned(),
        velocities: geometry_msgs::msg::TwistWithCovariance::into_rmw_message(std::borrow::Cow::Owned(msg.velocities)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        polygon: geometry_msgs::msg::Polygon::into_rmw_message(std::borrow::Cow::Borrowed(&msg.polygon)).into_owned(),
      radius: msg.radius,
      id: msg.id,
        orientation: geometry_msgs::msg::Quaternion::into_rmw_message(std::borrow::Cow::Borrowed(&msg.orientation)).into_owned(),
        velocities: geometry_msgs::msg::TwistWithCovariance::into_rmw_message(std::borrow::Cow::Borrowed(&msg.velocities)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      polygon: geometry_msgs::msg::Polygon::from_rmw_message(msg.polygon),
      radius: msg.radius,
      id: msg.id,
      orientation: geometry_msgs::msg::Quaternion::from_rmw_message(msg.orientation),
      velocities: geometry_msgs::msg::TwistWithCovariance::from_rmw_message(msg.velocities),
    }
  }
}


// Corresponds to costmap_converter_msgs__msg__ObstacleArrayMsg
/// Message that contains a list of polygon shaped obstacles.
/// Special types:
/// Polygon with 1 vertex: Point obstacle
/// Polygon with 2 vertices: Line obstacle
/// Polygon with more than 2 vertices: First and last points are assumed to be connected

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ObstacleArrayMsg {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub obstacles: Vec<super::msg::ObstacleMsg>,

}



impl Default for ObstacleArrayMsg {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ObstacleArrayMsg::default())
  }
}

impl rosidl_runtime_rs::Message for ObstacleArrayMsg {
  type RmwMsg = super::msg::rmw::ObstacleArrayMsg;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        obstacles: msg.obstacles
          .into_iter()
          .map(|elem| super::msg::ObstacleMsg::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        obstacles: msg.obstacles
          .iter()
          .map(|elem| super::msg::ObstacleMsg::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      obstacles: msg.obstacles
          .into_iter()
          .map(super::msg::ObstacleMsg::from_rmw_message)
          .collect(),
    }
  }
}


