#!/usr/bin/env python

# Copyright 2018 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.

"""
Unit testing for the I2C library.
"""

import app_api
import socket

DEFAULT_CONFIG_PATH = "/home/system/etc/config.toml"
SERVICE_MUTATION = (
    'mutation {testHardware(test:"Integration"){errors,status}}')
QUERY_TIMEOUT = 1.0  # Seconds


class IntegrationTest:

    def __init__(self, config_filepath=DEFAULT_CONFIG_PATH):
        self.api = app_api.Services(config_filepath)

    def test_services(self):
        for service in self.api.config:
            try:
                response = self.api.query(
                    service=service,
                    query=SERVICE_MUTATION,
                    timeout=QUERY_TIMEOUT)
                print "Status: SUCCESS\n {}".format(service)
                print "Response: {}\n".format(response)
            except socket.timeout as e:
                print "Status: FAILED\n {}".format(service)
                print "Error: No response from server"
                print " Timeout: {} seconds\n".format(QUERY_TIMEOUT)
            except Exception as e:
                print "Status: FAILED\n {}".format(service)
                print "Error: {}, {}\n".format(type(e), e)

    def test_service(self, service):
        try:
            response = self.api.query(
                service=service,
                query=SERVICE_MUTATION,
                timeout=QUERY_TIMEOUT)
            print "Status: SUCCESS\n {}".format(service)
            print "Response: {}\n".format(response)
        except socket.timeout as e:
            print "Status: FAILED\n {}".format(service)
            print "Error: No response from server"
            print " Timeout: {} seconds\n".format(QUERY_TIMEOUT)
        except Exception as e:
            print "Status: FAILED\n {}".format(service)
            print "Error: {}, {}\n".format(type(e), e)
