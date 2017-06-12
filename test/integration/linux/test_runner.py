#!/usr/bin/env python

import argparse
import logging
import json
import os
import re
import serial
import subprocess
import sys
import time
import tempfile
import urlparse

from collections import namedtuple


class TestUtils(object):
    GREEN  = '\033[92m'
    RED    = '\033[91m'
    NORMAL = '\033[0m'
    #TODO: Replace this class with the /tools/utils.py implementation

    def get_resource_type(self, obj):
        if urlparse.urlparse(obj).scheme != "":
            return 'url'
        elif os.path.exists(obj):
            return 'path'


    def get_kubos_repo_root(self):
        return os.path.normpath(os.path.abspath(__file__), '..', '..', '..')


    def clone_repo(self, url):
        temp_dir = tempfile.mkdtemp()
        os.chdir(temp_dir)
        self.run_cmd('git', 'clone', url)
        subdir = os.listdir(temp_dir)
        cloned_dir = os.path.join(temp_dir, subdir[0]) #there's only one sub-directory
        return cloned_dir


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
    MAX_SERIAL_READ_LEN = 500

    def __init__(self, config_file):
        self.setup_logger()
        self.logger = logging.getLogger()
        self.logger.setLevel(logging.INFO)
        self.load_config(config_file)
        self.test_summary = []


    def setup_serial_connection(self):
        self.device_path = '/dev/tty.usbserial-AH03B41R'
        self.serial_connection = serial.Serial(self.device_path, self.config.device['baudrate'])
        self.serial_connection.timeout = self.config.device['timeout'] if 'timeout' in self.config.device else 10


    def setup_logger(self):
        logger = logging.getLogger()
        fileHandler = logging.FileHandler("kubos_linux_test.log")
        logger.addHandler(fileHandler)
        consoleHandler = logging.StreamHandler()
        logger.addHandler(consoleHandler)


    def load_config(self, config_file):
        if not os.path.isfile(config_file):
            self.abort('The specified config file: %s does not exist.' % config_file)
        else:
            with open(config_file, 'r') as _file:
                data = json.loads(_file.read())
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
        self.print_test_summary()


    def run_test(self, test):
        test_data = self.dict_to_named_tuple(test)
        self.logger.info("Running test: %s" % test_data.name)

        if test_data.pre_test:
            self.logger.info("Test: %s Running pre-test command: %s\n" % (test_data.name, test_data.pre_test))
            self.run_cmd(test_data.pre_test)

        #build the project
        if test_data.build_source:
            resource_type = self.get_resource_type(test_data.build_source)
            logging.info("Test %s has type: %s" % (test, resource_type))
            if resource_type == 'url':
                proj_dir = self.clone_repo(test_data.build_source)
            elif resource_type == 'path':
                proj_dir = os.path.join(self.get_kubos_repo_root(), path)
            self.build_project(proj_dir)
            #Flash the project
            self.flash_project(proj_dir)

        #run the test command
        output = self.run_serial_command(test_data.test_command)
        self.verify_test_output(test, output)

        if test_data.post_test:
            self.logger.info("Test: %s Running post-test command: %s\n" % (test_data.name, test_data.post_test))
            self.run_cmd(test_data.post_test)


    def build_project(self, proj_dir):
        self.logger.info('Building Project: %s' % proj_dir)
        start_dir = os.getcwd()
        os.chdir(proj_dir)

        self.run_cmd('kubos', 'clean')
        self.run_cmd('kubos', 'link', '--all')
        ret_code = self.run_cmd('kubos', '-t', self.config.device['target'], 'build')
        if ret_code != 0:
            self.abort('Building project %s resulted in a non-zero exit code: %i.' % (proj_dir, ret_code))
        os.chdir(start_dir)


    def flash_project(self, project_dir):
        self.logger.info('Flashing Project: %s' % proj_dir)
        start_dir = os.getcwd()
        os.chdir(proj_dir)
        ret_code = self.run_cmd('kubos', '-t', self.config.device['target'], 'flash')
        if ret_code != 0:
            self.abort('Flashing project %s resulted in a non-zero exit code: %i.' % (proj_dir, ret_code))
        os.chdir(start_dir)


    def run_serial_command(self, command):
        '''
        sends a command string (command) over the serial device and reads the
        output until the next prompt (self.serial.promt) is read.
        '''
        self.serial_connection.write('%s\n' % str(command))
        output = self.serial_connection.read(self.MAX_SERIAL_READ_LEN).replace('\r', '')
        # parse the output
        command_len = len(command) + 1
        prompt_len = len(self.config.device['prompt']) + 1
        cmd_output = output[command_len : -prompt_len]
        return cmd_output


    def verify_test_output(self, test_data, actual):
        test_data = self.dict_to_named_tuple(test_data)
        if test_data.expected_regex:
            expected_regex = re.compile(test_data.expected_test_output)
            if expected_regex.match(actual):
                passed = True
        else:
            if actual == test_data.expected_test_output:
                passed = True

        if passed:
            self.add_test_success("Test %s passed" % test_data.name)
        else:
            self.abort('Test: %s Failed:\n Expected "%s" Did not match actual "%s"' % (test_data.name, test_data.expected_test_output, actual))


    def add_test_success(self, message):
        self.logger.info(message)
        self.test_summary.append('%s%s%s' % (self.GREEN, message, self.NORMAL))


    def login(self):
        self.logger.info('logging in')
        # self.serial_connection.write('%s\n' % str(self.config.login['username']))
        # time.sleep(3)
        # self.serial_connection.write('%s\n' % str(self.config.login['password']))


    def print_test_summary(self):
        self.logger.info('\n\nTest Summary:\n\n')
        for line in self.test_summary:
            self.logger.info(line)


    def abort(self, message):
        self.logger.error(message)
        sys.exit(1)


def main():
    config = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'test_config.json')
    runner = TestRunner(config)
    runner.setup_serial_connection()
    runner.run_tests()


if __name__ == "__main__":
    main()

