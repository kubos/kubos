#!/usr/bin/env python

# Copyright 2018 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details

"""
Runs Integration tests on all services included in service config file.

Records results in individual files named: {servicenname}__results.txt

Testing configuration toml is set up so that the system level config.toml
can be used if a test_command field is added to each service within that 
file.
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
                # Store service config
                service_config = self.test_config[service]
                # Check that there is a command for that service
                if "test_command" in service_config:
                    # Run test and store results in file
                    # Shell must be True to allow all full command line access
                    # Stores both stdout and stderr to the file
                    subprocess.call(service_config['test_command'],
                                    shell=True,
                                    stdout=service_result_file,
                                    stderr=subprocess.STDOUT)
                else:
                    # Note the lack of a test command in the results file
                    service_result_file.write(
                        "Service: "+service + " has no test_command\n")
            except Exception as e:
                # Log any errors encountered during testing in results file.
                # These are only errors that occur if the test fails to run.
                # Errors that are detected by testing are configured in each
                # service's test.
                service_result_file.write(
                    "Service: "+service +
                    "\n Failed to test with error: "+str(e))
                print e
            finally:
                # Ensures the file gets closed
                service_result_file.close()


if __name__ == '__main__':
    # Runs with defaults
    test = KubosServiceTest()
    test.runIntegrationTests()
