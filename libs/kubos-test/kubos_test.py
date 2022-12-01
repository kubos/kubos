#!/usr/bin/env python3

# Copyright 2018 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.

"""
Testing library for KubOS.

Currently only implements integration testing for hardware services.
"""

import kubos_app
import socket

DEFAULT_CONFIG_PATH = "/etc/kubos-config.toml"
SERVICE_MUTATION = (
    'mutation {test(test:NOOP){success,errors,results}}')
QUERY_TIMEOUT = 1.0  # Seconds


class IntegrationTest:

    def __init__(self,
                 config_filepath=DEFAULT_CONFIG_PATH):
        self.api = kubos_app.Services(config_filepath)

    def test_services(self, query=SERVICE_MUTATION):
        for service in self.api.config:
            self.test_service(service=service, query=query)

    def test_service(self, service, query=SERVICE_MUTATION):
        response = []
        try:
            # Complete the test mutation
            response = self.api.query(
                service=service,
                query=query,
                timeout=QUERY_TIMEOUT)

            # Check for successful test
            if response['test']['success']:
                print("Status : SUCCESS\n {}".format(service))
                print("Response : {}\n".format(response))
            else:
                print("Status : FAILED\n {}".format(service))
                print("Response : {}\n".format(response))
        except socket.timeout as e:
            print("Status : TIMEOUT\n {}".format(service))
            print("No response from server")
            print("Timeout : {} seconds\n".format(QUERY_TIMEOUT))
        except KeyError as e:
            print("Status : FORMAT ERROR\n {}".format(service))
            print("Service is sending back invalid response format")
            print("Error : {}, {}\n".format(type(e), e))
        except Exception as e:
            print("Status : TEST ERROR\n {}".format(service))
            print("Error : {}, {}\n".format(type(e), e))
