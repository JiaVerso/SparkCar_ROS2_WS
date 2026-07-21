from setuptools import find_packages
from setuptools import setup

setup(
    name='aslam_backend',
    version='1.0.0',
    packages=find_packages(
        include=('aslam_backend', 'aslam_backend.*')),
)
