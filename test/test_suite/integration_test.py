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


class KubosServiceTest:

    def __init__(self,
                 result_path=DEFAULT_RESULT_PATH,
                 test_config_path=TEST_CONFIG_PATH):

        self.test_config = toml.load(test_config_path)
        self.result_path = result_path

    def runIntegrationTests(self):
        for service in self.test_config:
            # Create and open results file
            filename = self.result_path + service
            with open(filename, "w+") as service_result_file:
                print "Running test for: ", service
                # Store service config
                service_config = self.test_config[service]
                # Check that there is a command for that service
                if "test_command" in service_config:
                    try:
                        output = subprocess.check_output(
                            service_config['test_command'],
                            stderr=subprocess.STDOUT,
                            shell=True,
                            universal_newlines=True)
                    except subprocess.CalledProcessError as exc:
                        # Subprocess encountered an error
                        print("Status : FAIL", exc.returncode, exc.output)
                        service_result_file.write("Status : FAIL\n")
                        service_result_file.write(exc.output)
                    else:
                        # Test ran successfully
                        print("Output: \n{}\n".format(output))
                        service_result_file.write(
                            "Output: \n{}\n".format(output))
                else:
                    # Note the lack of a test command
                    print("Status : FAIL", "No test_command field")
                    service_result_file.write("Status : FAIL\n")
                    service_result_file.write("No test_command field\n")


if __name__ == '__main__':
    # Runs with defaults
    test = KubosServiceTest()
    test.runIntegrationTests()
