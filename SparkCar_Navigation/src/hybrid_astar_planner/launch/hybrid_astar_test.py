from launch import LaunchDescription
from launch_ros.actions import Node


def generate_launch_description():
    return LaunchDescription([
        Node(
            package="hybrid_astar_planner",
            executable="a_star_demo",
            name="a_star_demo_node",
            output="screen",
        ),
    ])
