import os
from launch import LaunchDescription
from launch.actions import DeclareLaunchArgument
from launch.conditions import IfCondition
from launch.substitutions import PathJoinSubstitution, LaunchConfiguration
from launch_ros.actions import Node
from launch_ros.substitutions import FindPackageShare


def generate_launch_description():

    nav2_params_file = PathJoinSubstitution([
        FindPackageShare("sparkcar_nav_bringup"),
        "config",
        "nav2_params.yaml"
    ])

    map_yaml_file = PathJoinSubstitution([
        FindPackageShare("sparkcar_nav_bringup"),
        "maps",
        "main.yaml"
    ])

    rviz_config_file = PathJoinSubstitution([
        FindPackageShare("sparkcar_nav_bringup"),
        "rviz",
        "nav2.rviz"
    ])

    use_rviz = LaunchConfiguration("use_rviz")
    rviz_config = LaunchConfiguration("rviz_config")

    lifecycle_nodes = [
        'map_server',
        'planner_server',
        'controller_server',
        'smoother_server',
        'behavior_server',
        'bt_navigator'
    ]

    return LaunchDescription([

        DeclareLaunchArgument(
            "use_rviz",
            default_value="true",
            description="Whether to start RViz"
        ),

        DeclareLaunchArgument(
            "rviz_config",
            default_value=rviz_config_file,
            description="Full path to RViz config file"
        ),

        Node(
            package='nav2_map_server',
            executable='map_server',
            name='map_server',
            output='screen',
            parameters=[{'yaml_filename': map_yaml_file}, nav2_params_file]
        ),

        Node(
            package='nav2_planner',
            executable='planner_server',
            name='planner_server',
            output='screen',
            parameters=[nav2_params_file]
        ),

        Node(
            package='nav2_controller',
            executable='controller_server',
            name='controller_server',
            output='screen',
            parameters=[nav2_params_file]
        ),

        Node(
            package='nav2_smoother',
            executable='smoother_server',
            name='smoother_server',
            output='screen',
            parameters=[nav2_params_file]
        ),

        Node(
            package='nav2_behaviors',
            executable='behavior_server',
            name='behavior_server',
            output='screen',
            parameters=[nav2_params_file]
        ),

        Node(
            package='nav2_bt_navigator',
            executable='bt_navigator',
            name='bt_navigator',
            output='screen',
            parameters=[nav2_params_file]
        ),

        Node(
            package='nav2_lifecycle_manager',
            executable='lifecycle_manager',
            name='lifecycle_manager_navigation',
            output='screen',
            parameters=[
                {'use_sim_time': False},
                {'autostart': True},
                {'node_names': lifecycle_nodes}
            ]
        ),

        Node(
            package="rviz2",
            executable="rviz2",
            name="rviz2",
            output="screen",
            arguments=["-d", rviz_config],
            condition=IfCondition(use_rviz)
        ),
    ])