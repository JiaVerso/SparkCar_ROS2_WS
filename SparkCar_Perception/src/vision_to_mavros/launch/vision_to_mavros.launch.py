from launch import LaunchDescription
from launch.actions import DeclareLaunchArgument
from launch.substitutions import LaunchConfiguration
from launch_ros.actions import Node
from ament_index_python.packages import get_package_share_directory
import os


def generate_launch_description():
    default_config = os.path.join(
        get_package_share_directory('vision_to_mavros'),
        'config',
        'vision_to_mavros.yaml')

    config_arg = DeclareLaunchArgument(
        'config_file',
        default_value=default_config,
        description='Absolute path to the bridge parameter file')

    use_sim_time_arg = DeclareLaunchArgument(
        'use_sim_time', default_value='false')

    node = Node(
        package='vision_to_mavros',
        executable='vision_to_mavros_node',
        name='vision_to_mavros',
        output='screen',
        parameters=[
            LaunchConfiguration('config_file'),
            {'use_sim_time': LaunchConfiguration('use_sim_time')},
        ],
    )

    return LaunchDescription([config_arg, use_sim_time_arg, node])
