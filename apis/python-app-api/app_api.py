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
QUERY_TIMEOUT = 10.0  # Seconds
BLOCKING = 1  # 1 for blocking


class Services:

    def __init__(self, config_filepath=DEFAULT_CONFIG_PATH):
        self.config = toml.load(config_filepath)

    def query(self, service, query, timeout=QUERY_TIMEOUT):

        # Check inputs
        if service not in self.config:
            raise KeyError("Service not available.")
        if type(query) not in [str, unicode]:
            raise TypeError("Query must be str or unicode.")

        # Lookup port/ip
        ip = self.config[service]["addr"]["ip"]
        port = self.config[service]["addr"]["port"]

        # Talk to the server
        response = self._udp_query(query, (ip, port), timeout)

        # format the response and detect errors
        data = self._format(response, service)

        return data

    def _udp_query(self, query, (ip, port), timeout):
        try:
            # Set up the socket
            sock = socket.socket(socket.AF_INET,  # Internet
                                 socket.SOCK_DGRAM)  # UDP
            sock.setblocking(BLOCKING)
            sock.settimeout(timeout)
            sock.bind(("", 0))  # Binds to an available port

            # Send Query
            sock.sendto(query, (ip, port))

            # Wait for response (until timeout occurs)
            return sock.recv(UDP_BUFF)
        finally:
            sock.close()

    def _format(self, response, service):

        # Check and see if the length was maxed out
        if len(response) == UDP_BUFF:
            return response

        # Convert from JSON String if necessary
        if type(response) in [str, unicode]:
            response = json.loads(response)

        # Parse response according to GraphQL standard format
        data = response['msg']
        errors = response['errs']

        # Check for endpoint errors
        if errors not in ([], None):
            raise EnvironmentError(
                "{} Endpoint Error: {}".format(service, errors))

        return data
