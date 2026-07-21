from setuptools import find_packages
from setuptools import setup

setup(
    name='kalibr_camera_calibration',
    version='1.0.0',
    packages=find_packages(
        include=('kalibr_camera_calibration', 'kalibr_camera_calibration.*')),
)
