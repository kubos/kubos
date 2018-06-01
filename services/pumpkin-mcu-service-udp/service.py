#!/usr/bin/env python

# Copyright 2017 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.

"""
Boilerplate main for service application.
"""

from service import schema

from kubos_service import udp_service
from kubos_service.config import Config

c = Config("example-service")

# Set which modules are present and their addresses. 
schema.MODULES = c.raw['modules']

# Use this to start a udp service
udp_service.start(c, schema)
