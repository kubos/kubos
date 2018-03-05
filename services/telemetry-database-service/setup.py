#!/usr/bin/env python

from setuptools import setup

setup(name='Kubos-Telemetry-Database',
      version='1.0',
      description='Kubos Telemetry Database Service',
      author='Jesse Coffey',
      author_email='jcoffey@kubos.co',
      url='https://github.com/kubos/kubos/tree/master/services/telemetry-database-service',
      install_requires=[
          'graphene[sqlalchemy]',
          'SQLAlchemy==1.0.11',
          'Flask==0.10.1',
          'Flask-GraphQL==1.3.0',
          'graphene-sqlalchemy==2.0.0',
          'pyyaml==3.12'
      ],
     )
