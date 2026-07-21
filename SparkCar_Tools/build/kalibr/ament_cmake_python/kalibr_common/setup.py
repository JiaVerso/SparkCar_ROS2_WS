from setuptools import find_packages
from setuptools import setup

setup(
    name='kalibr_common',
    version='1.0.0',
    packages=find_packages(
        include=('kalibr_common', 'kalibr_common.*')),
)
