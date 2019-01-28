#!/usr/bin/env python

# Copyright 2018 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details

import argparse
import toml

DEFAULT_IP = "127.0.0.1"
DEFAULT_PORT = 8001
DEFAULT_PATH = "/home/system/etc/config.toml"


def get_args(name):
    parser = argparse.ArgumentParser(description=name)
    parser = argparse.ArgumentParser(description='Example Service')
    parser.add_argument("-c", "--config", type=str, help='path to config file')
    return parser.parse_args()


class Config:
    """Service configuration"""
    name = ""
    ip = ""
    port = 0
    raw = {}

    def __init__(self, name):
        args = get_args(name)
        if args.config:
            path = args.config
        else:
            path = DEFAULT_PATH
        try:
            data = toml.load(path)
            self.name = name
            self.ip = data[name]['addr']['ip']
            self.port = data[name]['addr']['port']
            self.raw = data[name]

        except Exception:
            self.ip = DEFAULT_IP
            self.port = DEFAULT_PORT
