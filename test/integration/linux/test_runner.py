#!/usr/bin/env python

import argparse
import logging
import json
import os
import serial
import sys

from collections import namedtuple

class TestRunner(object):
    def __init__(self, config_file):
        self.setup_logger()
        self.logger = logging.getLogger()
        self.logger.setLevel(logging.INFO)
        self.config = self.load_config(config_file)


    def setup_logger(self):
        logger = logging.getLogger()
        fileHandler = logging.FileHandler("kubos_linux_test.log")
        logger.addHandler(fileHandler)
        consoleHandler = logging.StreamHandler()
        logger.addHandler(consoleHandler)


    def load_config(self, config_file):
        if not os.path.isfile(config_file):
            self.abort('The specified config file: %s does not exist.')
        else:
            with open(config_file, 'r') as _file:
                data = json.loads(_file.read())
                self.logger.info("Loaded config: %s" %data)
            return dict_to_named_tuple


    def dict_to_named_tuple(self, dictionary)
        '''
        This allows dictionaries to be accessed via dot notation
        example:

        dictionary['key'] -> dictionary.key
        '''
        return namedtuple("Config", dictionary.keys())(*dictionary.values())


    def run_tests(self):
        if self.config.login:
            self.login()
        for test in self.config.tests:
            run_test(test)


    def login(self):
        self.logger.info('logging in')


    def abort(self, message):
        self.logger.error(message)
        sys.exit(1)


def main():
    config = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'test_config.json')
    runner = TestRunner(config)
    runner.run_tests()


if __name__ == "__main__":
    main()

