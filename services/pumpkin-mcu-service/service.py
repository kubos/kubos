#!/usr/bin/env python

# Copyright 2018 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.

"""
main for Pumpkin MCU service
"""

import logging
from logging.handlers import SysLogHandler

from service import schema

from kubos_service import udp_service
from kubos_service.config import Config

c = Config("pumpkin-mcu-service")

# Setup logging
logger = logging.getLogger(c.name)
logger.setLevel(logging.DEBUG)
handler = SysLogHandler(address='/dev/log', facility=SysLogHandler.LOG_DAEMON)
formatter = logging.Formatter('pumpkin-mcu-service: %(message)s')
handler.formatter = formatter
logger.addHandler(handler)

# Set which modules are present and their addresses from the config file.
schema.MODULES = c.raw['modules']

# Starts the UDP service
udp_service.start(c, schema)
