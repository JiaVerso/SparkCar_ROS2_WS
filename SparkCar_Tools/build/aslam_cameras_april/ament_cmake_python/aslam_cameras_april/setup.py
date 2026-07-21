from setuptools import find_packages
from setuptools import setup

setup(
    name='aslam_cameras_april',
    version='1.0.0',
    packages=find_packages(
        include=('aslam_cameras_april', 'aslam_cameras_april.*')),
)
