from setuptools import find_packages
from setuptools import setup

setup(
    name='kalibr_imu_camera_calibration',
    version='1.0.0',
    packages=find_packages(
        include=('kalibr_imu_camera_calibration', 'kalibr_imu_camera_calibration.*')),
)
