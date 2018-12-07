#!/usr/bin/env python

# Copyright 2017 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.

"""
Boilerplate main for service application.
"""

import logging

from service import schema
from kubos_service.config import Config
from logging.handlers import SysLogHandler

config = Config("example-service")

# Setup logging
logger = logging.getLogger("example-service")
logger.setLevel(logging.DEBUG)
handler = SysLogHandler(address='/dev/log', facility=SysLogHandler.LOG_DAEMON)
formatter = logging.Formatter('example-service: %(message)s')
handler.formatter = formatter
logger.addHandler(handler)

# from kubos_service import http_service
# Start an http service
# http_service.start(config, schema.schema)

from kubos_service import udp_service

# Start a udp service with optional context
# udp_service.start(config, schema, {'bus': '/dev/ttyS3'})

# Start a udp service
udp_service.start(config, schema)
