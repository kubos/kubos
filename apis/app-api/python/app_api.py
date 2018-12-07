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

SERVICE_CONFIG_PATH = "/home/system/etc/config.toml"
UDP_BUFF_LEN = 1024
DEFAULT_TIMEOUT = 10.0  # Seconds


class Services:

    def __init__(self, service_config_filepath=SERVICE_CONFIG_PATH):
        self.config = toml.load(service_config_filepath)

    def query(self, service, query, timeout=DEFAULT_TIMEOUT):

        # Check inputs
        if service not in self.config:
            raise KeyError(
                "Service name invalid. Check config file for service names.")
        if type(query) not in [str, unicode]:
            raise TypeError("Query must be str or unicode.")

        # Lookup port/ip
        ip = self.config[service]["addr"]["ip"]
        port = self.config[service]["addr"]["port"]

        # Talk to the server
        response = self._udp_query(query, (ip, port), timeout)

        # Format the response and detect errors
        (data, errors) = self._format(response, service)

        # Check for endpoint errors
        if errors not in ([], None, ""):
            raise EnvironmentError(
                "{} Endpoint Error: {}".format(service, errors))

        return data

    def _udp_query(self, query, (ip, port), timeout):
        # Set up the socket
        sock = socket.socket(socket.AF_INET,  # Internet
                             socket.SOCK_DGRAM)  # UDP
        try:
            sock.settimeout(timeout)
            sock.bind(("", 0))  # Binds to an available port

            # Send Query
            sock.sendto(query, (ip, port))

            # Wait for response (until timeout occurs)
            response = sock.recv(UDP_BUFF_LEN)
            return response
        finally:
            sock.close()

    def _format(self, response, service):

        # Parse JSON response
        try:
            response = json.loads(response)
        except Exception as e:
            print("Response was unable to be parsed as JSON.")
            print("It is likely incomplete or the endpoint is misbehaving")
            print("response: ", response)
            print("error: ", e)
            raise

        # Check that it follows GraphQL format
        if 'data' not in response or 'errors' not in response:
            raise KeyError(
                "{} Endpoint Error: ".format(service) +
                "Response contains incorrect fields: \n{}".format(response))

        # Parse response according to GraphQL standard format
        data = response['data']
        errors = response['errors']

        return (data, errors)
