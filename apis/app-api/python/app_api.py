#!/usr/bin/env python

# Copyright 2018 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.

"""
Mission Application API for Python Mission Applications.
"""

import json
import logging
from logging.handlers import SysLogHandler
import socket
import sys
import toml

SERVICE_CONFIG_PATH = "/home/system/etc/config.toml"
UDP_BUFF_LEN = 1024
DEFAULT_TIMEOUT = 10.0  # Seconds


class Services:

    def __init__(self, service_config_filepath=SERVICE_CONFIG_PATH):
        self.config = toml.load(service_config_filepath)

    def query(self, service, query, timeout=DEFAULT_TIMEOUT):
        """Send a GraphQL request to a service
        
        Args:
        
            - service (str): The service that the request should be sent to. Must be defined in
              the system's ``config.toml`` file
            - query (str): The GraphQL request
            - timeout (int): The amount of time that this function should wait for a response from the
              service
        
        Returns:
            The JSON response from the service
            
        Raises:
            EnvironmentError: An error was returned within the JSON response from the service
            KeyError: The `service` value was invalid
            TimeoutError: The function timed out while waiting for a response from teh service
            TypeError: The `query` value was invalid
            
        """
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
        if 'errors' not in response:
            raise KeyError(
                "{} Endpoint Error: ".format(service) +
                "Response contains incorrect fields: \n{}".format(response))

        # Parse response according to GraphQL standard format
        data = response['data']
        errors = response['errors']

        return (data, errors)

def logging_setup(app_name):
    """Set up the logger for the program
    All log messages will be sent to rsyslog using the User facility.
    Additionally, they will also be echoed to ``stdout``
    
    Args:
    
        - app_name (:obj:`str`): The application name which should be used for all log messages

    Returns:
        An initialized Logger object
    """
    # Create a new logger
    logger = logging.getLogger(app_name)
    # We'll log everything of Debug level or higher
    logger.setLevel(logging.DEBUG)
    # Set the log message template
    formatter = logging.Formatter(app_name + ' %(message)s')
    
    # Set up a handler for logging to syslog
    syslog = SysLogHandler(address='/dev/log', facility=SysLogHandler.LOG_USER)
    syslog.setFormatter(formatter)
    
    # Set up a handler for logging to stdout
    stdout = logging.StreamHandler(stream=sys.stdout)
    stdout.setFormatter(formatter)
    
    # Finally, add our handlers to our logger
    logger.addHandler(syslog)
    logger.addHandler(stdout)
    
    return logger