#!/usr/bin/env python

# Copyright 2017 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.

"""
Boilerplate main for service application.
"""

from service import app
import json

# I know this is hacky...I need help with how to manage config files
import os.path
config_filename = 'service_config.json'
config_path = os.path.abspath(os.path.dirname(__file__)) + '/' + config_filename
with open(config_path) as config_file:
    CONFIG_DATA = json.load(config_file)

app = app.create_app()
app.run(host=CONFIG_DATA['APP_IP'], port=CONFIG_DATA['APP_PORT'])
