#!/usr/bin/env python

# Copyright 2018 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.

"""
Boilerplate main for service application.
"""

from service import app

# Service Configuration Variables
APP_IP = "0.0.0.0"
APP_PORT = 5000

app = app.create_app()
app.run(host=APP_IP, port=APP_PORT)
