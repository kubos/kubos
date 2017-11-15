#!/usr/bin/env python

# Copyright 2017 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.

"""
Boilerplate main for handler application.
"""

import argparse
from handler import app
import yaml

parser = argparse.ArgumentParser(description='Example Handler')
parser.add_argument('config', type=str, help='path to config file')
args = parser.parse_args()

with open(args.config) as ymlfile:
    cfg = yaml.load(ymlfile)

app = app.create_app()
app.run(host=cfg['APP_IP'], port=cfg['APP_PORT'])
