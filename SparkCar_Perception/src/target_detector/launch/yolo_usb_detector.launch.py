import os

from ament_index_python.packages import get_package_share_directory
from launch import LaunchDescription
from launch.actions import DeclareLaunchArgument
from launch.substitutions import LaunchConfiguration
from launch_ros.actions import Node
from launch_ros.parameter_descriptions import ParameterValue


def generate_launch_description():
    package_share = get_package_share_directory('target_detector')
    default_model = os.path.join(package_share, 'models', 'yolo11n.torchscript')
    default_config = os.path.join(package_share, 'config', 'yolo_usb_detector.yaml')

    return LaunchDescription([
        DeclareLaunchArgument(
            'model_path', default_value=default_model,
            description='Path to a YOLO11 TorchScript detection model',
        ),
        DeclareLaunchArgument(
            'camera_backend', default_value='realsense',
            description="Camera backend: 'realsense' or 'v4l2'",
        ),
        DeclareLaunchArgument(
            'camera_device', default_value='/dev/video4',
            description='V4L2 USB camera device',
        ),
        DeclareLaunchArgument(
            'target_class_id', default_value='0',
            description='Only detections with this class ID are published',
        ),
        Node(
            package='target_detector',
            executable='yolo_usb_detector',
            name='yolo_usb_detector',
            output='screen',
            parameters=[
                default_config,
                {
                    'model_path': ParameterValue(
                        LaunchConfiguration('model_path'), value_type=str
                    ),
                    'camera_backend': ParameterValue(
                        LaunchConfiguration('camera_backend'), value_type=str
                    ),
                    'camera_device': ParameterValue(
                        LaunchConfiguration('camera_device'), value_type=str
                    ),
                    'target_class_id': ParameterValue(
                        LaunchConfiguration('target_class_id'), value_type=int
                    ),
                },
            ],
        ),
    ])
