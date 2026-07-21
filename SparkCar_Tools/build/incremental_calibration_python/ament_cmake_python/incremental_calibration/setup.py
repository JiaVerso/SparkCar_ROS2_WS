from setuptools import find_packages
from setuptools import setup

setup(
    name='incremental_calibration',
    version='1.0.0',
    packages=find_packages(
        include=('incremental_calibration', 'incremental_calibration.*')),
)
