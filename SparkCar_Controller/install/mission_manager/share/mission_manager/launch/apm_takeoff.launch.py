import os

from ament_index_python.packages import get_package_share_directory
from launch import LaunchDescription
from launch_ros.actions import Node


def generate_launch_description():
    package_share = get_package_share_directory("mission_manager")

    config_file = os.path.join(
        package_share,
        "config",
        "apm_takeoff.yaml",
    )

    return LaunchDescription([
        Node(
            package="mission_manager",
            executable="apm_takeoff_node",
            name="apm_takeoff_node",
            output="screen",
            parameters=[config_file],
        )
    ])