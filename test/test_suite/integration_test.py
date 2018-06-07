#!/usr/bin/env python

# Copyright 2018 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details

"""
Runs Integration tests on all services included in service config file.
"""

import subprocess
import toml

DEFAULT_CONFIG_PATH = "/home/system/etc/config.toml"
DEFAULT_RESULT_PATH = "/home/system/etc/integration_test_results.txt"
TEST_CONFIG_PATH = "testing_config.toml"


class KubosServiceTest:

    def __init__(self,
                 config_path=DEFAULT_CONFIG_PATH,
                 result_path=DEFAULT_RESULT_PATH):
        self.config_data = toml.load(config_path)
        self.result_file = open(result_path, "w+")

    def runIntegrationTests(filepath):
        for service in self.config_data:
            try:
                subprocess.
            except Exception as e:
                print e


if __name__ == '__main__':
    test = KubosServiceTest(
        config_path="/Users/jessecoffey/Code/apollo-fusion" +
        "/common/overlay/home/system/etc/config.toml",
        result_path="/Users/jessecoffey/Code/integration_test_results.txt")
