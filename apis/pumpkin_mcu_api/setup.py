#!/usr/bin/env python
"""
A setuptools based setup module for the Pumpkin MCU API.
See:
https://github.com/pypa/sampleproject
"""

from setuptools import setup

setup(name='pumpkin_mcu',
      version='0.1.0',
      license='MIT'
      description='KubOS API for communicating with Pumpkin module MCUs',
      packages=["mcu_api"],
      )