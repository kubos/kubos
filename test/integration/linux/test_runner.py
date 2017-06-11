#!/usr/bin/env python

import argparse
import logging
import json
import os
import serial
import sys
import urlparse

from collections import namedtuple


class TestUtils(object):

    def get_resource_type(self, obj):
        if urlparse.urlparse(obj).scheme != "":
            return 'url'
        elif os.path.exists(obj):
            return 'path'

    def get_kubos_repo_root(self):
        return os.path.normpath(os.path.abspath(__file__), '..', '..', '..')


    def clone_repo(self, url):
        pass

    def run_cmd(self, *args, **kwargs):
        cwd = kwargs.get('cwd', os.getcwd())
        save_output = kwargs.pop('save_output', False)
        echo = kwargs.pop('echo', True)

        if echo:
            print ' '.join(args)
        try:
            if save_output:
                return subprocess.check_output(args, **kwargs)
            else:
                return subprocess.check_call(args, **kwargs)
        except subprocess.CalledProcessError, e:
            if echo:
                print >>sys.stderr, 'Error executing command, giving up'
            return 1



class TestRunner(TestUtils):
    def __init__(self, config_file):
        self.setup_logger()
        self.logger = logging.getLogger()
        self.logger.setLevel(logging.INFO)
        self.load_config(config_file)


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
                self.logger.info("Loaded config: %s" % data)
            self.config = self.dict_to_named_tuple(data)


    def dict_to_named_tuple(self, dictionary):
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
            self.run_test(test)


    def run_test(self, test):
        test_data = self.dict_to_named_tuple(test)
        self.logger.info("Running test: %s" % test_data.name)

        #build the project
        if test_data.build:
            resource_type = get_resource_type(test_data.build)
            logging.info("Test %s has type: %s" % (test, resource_type))
            if resource_type == 'url':
                proj_dir = self.clone_repo(url)
            elif resource_type == 'path':
                proj_dir = os.path.join(self.get_kubos_repo_root(), path)
            self.build_project(proj_dir)
            #Flash the project
            self.flash_project(proj_dir)

        if test_data.test_command:
            output = self.run_serial_command(test_data.test_command)
            




    def build_project(self, proj_dir):
        self.logger.info('Building Project: %s' % proj_dir)
        start_dir = os.getcwd()
        os.chdir(proj_dir)
        ret_code = self.run_cmd('kubos', 'clean', '&&', 'kubos', 'build', '-t', self.config.device['target']])
        if ret_code != 0:
            self.abort('Building project %s resulted in a non-zero exit code: %i.' % (proj_dir, ret_code))
        os.chdir(start_dir)


    def flash_project(self, project_dir):
        self.logger.info('Flashing Project: %s' % proj_dir)
        start_dir = os.getcwd()
        os.chdir(proj_dir)
        ret_code = self.run_cmd('kubos', 'flash', '-t', self.config.device['target']])
        if ret_code != 0:
            self.abort('Building project %s resulted in a non-zero exit code: %i.' % (proj_dir, ret_code))
        os.chdir(start_dir)


    def run_serial_command(self, command):
        '''
        sends a command string (command) over the serial device and reads the
        output until the next prompt (self.serial.promt) is read.
        '''
        pass
        #finish this

    def login(self):
        self.logger.info('logging in')
        #finish this


    def abort(self, message):
        self.logger.error(message)
        sys.exit(1)


def main():
    config = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'test_config.json')
    runner = TestRunner(config)
    runner.run_tests()


if __name__ == "__main__":
    main()

