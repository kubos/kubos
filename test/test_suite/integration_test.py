#!/usr/bin/env python

# Copyright 2018 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details

"""
Runs Integration tests on all services included in service config file.

Records results in individual files named: {servicenname}__results.txt.
"""

import subprocess
import toml

DEFAULT_RESULT_PATH = "results/"
TEST_CONFIG_PATH = "testing_config.toml"
RESULTS_FILE_APPEND = "__results"


class KubosServiceTest:

    def __init__(self,
                 result_path=DEFAULT_RESULT_PATH,
                 test_config_path=TEST_CONFIG_PATH):

        self.test_config = toml.load(test_config_path)
        self.result_path = result_path

    def runIntegrationTests(self):
        for service in self.test_config:
            # Create and open results file
            service_result_file = open(
                self.result_path+service+RESULTS_FILE_APPEND, "w+")
            try:
                #
                service_config = self.test_config[service]
                if "test_command" in service_config:
                    subprocess.call(service_config['test_command'],
                                    shell=True,
                                    stdout=service_result_file,
                                    stderr=subprocess.STDOUT)
                else:
                    service_result_file.write(
                        "Service: "+service + " has no test_command\n")
            except Exception as e:
                service_result_file.write(
                    "Service: "+service +
                    "\n Failed to test with error: "+str(e))
                print e
            finally:
                service_result_file.close()


if __name__ == '__main__':
    test = KubosServiceTest()
    test.runIntegrationTests()
