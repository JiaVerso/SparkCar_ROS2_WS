import os

from ament_index_python.packages import get_package_share_directory
from launch import LaunchDescription
from launch.actions import DeclareLaunchArgument
from launch.substitutions import LaunchConfiguration
from launch_ros.actions import Node
from launch_ros.parameter_descriptions import ParameterValue


def generate_launch_description():
    package_share = get_package_share_directory('target_follower')
    config_file = os.path.join(package_share, 'config', 'target_follower.yaml')

    return LaunchDescription([
        DeclareLaunchArgument(
            'desired_distance', default_value='1.2',
            description='Desired person distance in meters',
        ),
        DeclareLaunchArgument(
            'start_enabled', default_value='false',
            description='Start moving as soon as valid target messages arrive',
        ),
        DeclareLaunchArgument(
            'allow_reverse', default_value='false',
            description='Allow reversing when the person is closer than the desired distance',
        ),
        Node(
            package='target_follower',
            executable='target_follower_node',
            name='target_follower_node',
            output='screen',
            parameters=[
                config_file,
                {
                    'desired_distance': ParameterValue(
                        LaunchConfiguration('desired_distance'), value_type=float
                    ),
                    'start_enabled': ParameterValue(
                        LaunchConfiguration('start_enabled'), value_type=bool
                    ),
                    'allow_reverse': ParameterValue(
                        LaunchConfiguration('allow_reverse'), value_type=bool
                    ),
                },
            ],
        ),
    ])
