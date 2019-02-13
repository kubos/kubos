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
import requests
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
        if type(query) not in [str, bytes]:
            raise TypeError("Query must be str or bytes.")
        
        if type(query) is bytes:
            query = query.decode()

        # Lookup port/ip
        ip = self.config[service]["addr"]["ip"]
        port = self.config[service]["addr"]["port"]

        # Talk to the server
        response = self._http_query(query, ip, port, timeout)

        # Format the response and detect errors
        (data, errors) = self._format(response, service)

        # Check for endpoint errors
        if errors not in ([], None, ""):
            raise EnvironmentError(
                "{} Endpoint Error: {}".format(service, errors))

        return data

    def _http_query(self, query, ip, port, timeout):

        # Service connection info
        url = "http://{}:{}".format(ip, port)

        # Put our query in the message body as JSON
        body = {'query':query} 
        
        # Send the request and wait for the response
        response = requests.post(str.encode(url), json=body, timeout=timeout)
        
        # Make sure that we got a good response
        response.raise_for_status()

        # Return the good message body
        return response.text

    def _format(self, response, service):

        # Parse JSON response
        try:
            response = json.loads(response)
        except Exception as e:
            print("Response was unable to be parsed as JSON.")
            print("It is likely incomplete or the endpoint is misbehaving")
            print("response: {}".format(response))
            print("error: {}".format(e))
            raise

        # Check that it follows GraphQL format
        data = ""
        errors = ""

        for key,value in response.items():
            if key not in ['data', 'errors']:
                raise KeyError(
                    "{} Endpoint Error: ".format(service) +
                    "Response contains incorrect fields: \n{}".format(response))

        if 'errors' in response:
            # Collect any errors
            errors = response['errors']

        if 'data' in response:
            data = response['data']
        else:
            # If the 'data' field isn't returned, there *must* be at least one error message
            if errors == "":
                raise KeyError(
                    "{} Endpoint Error: ".format(service) +
                    "Response contains incorrect fields: \n{}".format(response))

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