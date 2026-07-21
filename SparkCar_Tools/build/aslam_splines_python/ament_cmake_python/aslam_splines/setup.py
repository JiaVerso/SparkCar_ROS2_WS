from setuptools import find_packages
from setuptools import setup

setup(
    name='aslam_splines',
    version='1.0.0',
    packages=find_packages(
        include=('aslam_splines', 'aslam_splines.*')),
)
