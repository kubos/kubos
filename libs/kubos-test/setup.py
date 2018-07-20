#!/usr/bin/env python
"""
A setuptools based setup module for the kubos test package.
See:
https://github.com/pypa/sampleproject
"""

from setuptools import setup

setup(name='kubos_test',
      version='0.1.0',
      description='Manual integration testing library for KubOS Services',
      py_modules=["kubos_test"],
      install_requires=["app_api"]
      )
