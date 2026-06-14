#!/usr/bin/env python3

import math
import sys

from action_msgs.msg import GoalStatus
from ament_index_python.packages import get_package_share_directory
from geometry_msgs.msg import PoseStamped
from nav2_msgs.action import NavigateThroughPoses
import rclpy
from rclpy.action import ActionClient
from rclpy.node import Node
import yaml


def default_waypoints_file():
    return (
        get_package_share_directory("sparkcar_nav_bringup")
        + "/config/waypoints.yaml"
    )


def yaw_to_quaternion(yaw):
    half_yaw = yaw * 0.5
    return math.sin(half_yaw), math.cos(half_yaw)


class WaypointControl(Node):
    def __init__(self):
        super().__init__("waypoint_control")

        self.declare_parameter("waypoints_file", default_waypoints_file())
        self.waypoints_file = self.get_parameter("waypoints_file").value

        config = self.load_config(self.waypoints_file)
        node_config = config.get("waypoint_control", {})

        self.action_name = node_config.get("action_name", "navigate_through_poses")
        self.loop = bool(node_config.get("loop", False))
        self.repeat_count = int(node_config.get("repeat_count", 1))
        self.default_frame_id = node_config.get("frame_id", "map")
        self.behavior_tree = node_config.get("behavior_tree", "")

        self.waypoints = config.get("waypoints", [])
        if not self.waypoints:
            self.get_logger().error("No waypoints found in YAML file.")
            sys.exit(1)

        self.last_poses_remaining = None

        self.action_client = ActionClient(
            self,
            NavigateThroughPoses,
            self.action_name,
        )

    def load_config(self, path):
        with open(path, "r", encoding="utf-8") as file:
            data = yaml.safe_load(file)

        if data is None:
            raise RuntimeError(f"Empty YAML file: {path}")

        return data

    def waypoint_to_pose(self, waypoint):
        pose = PoseStamped()
        pose.header.frame_id = waypoint.get("frame_id", self.default_frame_id)
        pose.header.stamp = self.get_clock().now().to_msg()

        pose.pose.position.x = float(waypoint["x"])
        pose.pose.position.y = float(waypoint["y"])
        pose.pose.position.z = float(waypoint.get("z", 0.0))

        if "yaw" in waypoint:
            qz, qw = yaw_to_quaternion(float(waypoint["yaw"]))
            pose.pose.orientation.z = qz
            pose.pose.orientation.w = qw
        else:
            pose.pose.orientation.z = float(waypoint.get("qz", 0.0))
            pose.pose.orientation.w = float(waypoint.get("qw", 1.0))

        return pose

    def build_goal(self):
        goal = NavigateThroughPoses.Goal()
        goal.poses = [self.waypoint_to_pose(point) for point in self.waypoints]

        if hasattr(goal, "behavior_tree"):
            goal.behavior_tree = self.behavior_tree

        return goal

    def feedback_callback(self, feedback_msg):
        feedback = feedback_msg.feedback
        poses_remaining = getattr(feedback, "number_of_poses_remaining", None)

        if poses_remaining is not None and poses_remaining != self.last_poses_remaining:
            self.last_poses_remaining = poses_remaining
            self.get_logger().info(
                f"NavigateThroughPoses feedback: poses remaining={poses_remaining}"
            )

    def send_through_poses_once(self):
        goal = self.build_goal()
        self.get_logger().info(
            f"Sending {len(goal.poses)} poses to /{self.action_name}."
        )

        send_future = self.action_client.send_goal_async(
            goal,
            feedback_callback=self.feedback_callback,
        )
        rclpy.spin_until_future_complete(self, send_future)

        goal_handle = send_future.result()
        if goal_handle is None or not goal_handle.accepted:
            self.get_logger().error("NavigateThroughPoses goal was rejected.")
            return False

        self.get_logger().info("NavigateThroughPoses goal accepted.")

        result_future = goal_handle.get_result_async()
        rclpy.spin_until_future_complete(self, result_future)

        result = result_future.result()
        if result.status == GoalStatus.STATUS_SUCCEEDED:
            self.get_logger().info("All waypoints completed.")
            return True

        self.get_logger().warn(
            f"NavigateThroughPoses failed, action status={result.status}."
        )
        return False

    def run(self):
        self.get_logger().info(f"Loading waypoints from: {self.waypoints_file}")
        self.get_logger().info(f"Waiting for /{self.action_name} action server...")
        self.action_client.wait_for_server()

        rounds = 0
        while rclpy.ok():
            rounds += 1
            self.get_logger().info(f"Waypoint control round {rounds} started.")
            self.send_through_poses_once()

            if self.loop:
                continue

            if rounds >= self.repeat_count:
                break

        self.get_logger().info("Waypoint control finished.")


def main():
    rclpy.init()
    node = WaypointControl()
    node.run()
    node.destroy_node()
    rclpy.shutdown()


if __name__ == "__main__":
    main()
