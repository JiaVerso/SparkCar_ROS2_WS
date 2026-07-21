from setuptools import find_packages
from setuptools import setup

setup(
    name='kalibr',
    version='1.0.0',
    packages=find_packages(
        include=('kalibr', 'kalibr.*')),
)
