#!/usr/bin/env python

# Copyright 2017 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.

"""
Boilerplate main for service application.
"""

from service import schema
from kubos_service import http_service
from kubos_service import udp_service
from kubos_service.config import Config

config = Config("example-service")

# Start an http service
# http_service.start(config, schema.schema)

# Start a udp service with optional context
# udp_service.start(config, schema, {'bus': '/dev/ttyS3'})

# Start a udp service
udp_service.start(config, schema)
