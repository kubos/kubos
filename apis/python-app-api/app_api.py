#!/usr/bin/env python

# Copyright 2018 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.

"""
Mission Application API for Python Mission Applications.
"""

import toml
import socket
import json

DEFAULT_CONFIG_PATH = "/home/system/etc/config.toml"
UDP_BUFF = 1024


class Services:
    def __init__(config_filepath=DEFAULT_CONFIG_PATH):
        self.config = toml.load(config_filepath)
        self.sock = socket.socket(socket.AF_INET,  # Internet
                                  socket.SOCK_DGRAM)  # UDP
        self.sock.bind(("", 0))  # Binds to a local socket

    def service_query(self, service, query):
        ip = self.config[service]["addr"]["ip"]
        port = self.config[service]["addr"]["port"]
        self.sock.sendto(query, (ip, port))
        response, _ = self.sock.recvfrom(UDP_BUFF)
        return json.loads(response)
