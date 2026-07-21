from setuptools import find_packages
from setuptools import setup

setup(
    name='numpy_eigen',
    version='1.0.0',
    packages=find_packages(
        include=('numpy_eigen', 'numpy_eigen.*')),
)
