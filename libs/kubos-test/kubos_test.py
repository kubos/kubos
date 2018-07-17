#!/usr/bin/env python

# Copyright 2018 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.

"""
Testing library for KubOS.

Currently only implements integration testing for hardware services.
"""

import app_api
import socket
import json

DEFAULT_CONFIG_PATH = "/home/system/etc/config.toml"
SERVICE_MUTATION = (
    'mutation {testHardware(test:INTEGRATION){errors,success}}')
QUERY_TIMEOUT = 1.0  # Seconds


class IntegrationTest:

    def __init__(self,
                 config_filepath=DEFAULT_CONFIG_PATH):
        self.api = app_api.Services(config_filepath)

    def test_services(self):
        for service in self.api.config:
            self.test_service(service=service)

    def test_service(self, service):
        try:
            # Complete the test mutation
            response = self.api.query(
                service=service,
                query=SERVICE_MUTATION,
                timeout=QUERY_TIMEOUT)

            # Check for successful test
            if response['testHardware']['success']:
                print "Status : SUCCESS\n {}".format(service)
                print "Response : {}\n".format(response)
            else:
                print "Status : FAILED\n {}".format(service)
                print "Response : {}\n".format(response)
        except socket.timeout as e:
            print "Status : TIMEOUT\n {}".format(service)
            print "No response from server"
            print "Timeout : {} seconds\n".format(QUERY_TIMEOUT)
        except KeyError as e:
            print "Status : FORMAT ERROR\n {}".format(service)
            print "Service is sending back invalid response format"
            print "Response : {}\n".format(response)
            print "Error : {}, {}\n".format(type(e), e)
        except Exception as e:
            print "Status : TEST ERROR\n {}".format(service)
            print "Error : {}, {}\n".format(type(e), e)
