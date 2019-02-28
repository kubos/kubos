#!/usr/bin/env python

# Copyright 2018 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.

"""
main for Pumpkin MCU service
"""

import logging
from logging.handlers import SysLogHandler
import sys

from service import schema

from kubos_service import http_service
from kubos_service.config import Config

c = Config("pumpkin-mcu-service")

# Setup logging
logger = logging.getLogger("pumpkin-mcu-service")
logger.setLevel(logging.DEBUG)
handler = SysLogHandler(address='/dev/log', facility=SysLogHandler.LOG_DAEMON)
formatter = logging.Formatter('pumpkin-mcu-service: %(message)s')
handler.formatter = formatter
logger.addHandler(handler)

# Set up a handler for logging to stdout
stdout = logging.StreamHandler(stream=sys.stdout)
stdout.setFormatter(formatter)
logger.addHandler(stdout)

# Set which modules are present and their addresses from the config file.
schema.MODULES = c.raw['modules']

# Starts the HTTP service
http_service.start(c, schema.schema)
