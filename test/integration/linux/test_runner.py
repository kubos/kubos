#!/usr/bin/env python2

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

import config as kubos_config


class TestUtils(object):
    GREEN  = '\033[92m'
    RED    = '\033[91m'
    NORMAL = '\033[0m'
    # TODO: Replace this class with the /tools/utils.py implementation

    def get_abs_path(self, base_dir, path):
        abs_path = os.path.normpath(os.path.join(base_dir, path))
        self.logger.info('Interperating provided path as relative. Using %s' % abs_path)
        return abs_path


    def clone_repo(self, url):
        temp_dir = tempfile.mkdtemp()
        os.chdir(temp_dir)
        self.run_cmd('git', 'clone', url)
        subdir = os.listdir(temp_dir)
        cloned_dir = os.path.join(temp_dir, subdir[0]) # There's only one sub-directory
        return cloned_dir


    def get_flash_project_dir(self):
        '''
        Instead of creating a project to flash non-project files, just use the hello-world project
        '''
        this_dir = os.path.dirname(os.path.abspath(__file__))
        flash_proj_dir = os.path.join(this_dir, 'hello-world')
        self.build_flash_proj(flash_proj_dir)
        return flash_proj_dir


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

    def __init__(self, config_file, specified_tests):
        self.load_configuration(config_file)
        self.setup_logger()
        self.logger = logging.getLogger()
        self.logger.setLevel(logging.INFO)
        self.test_summary = []
        if specified_tests is not None:
            self.remove_non_specified_tests(specified_tests)


    def setup_serial_connection(self, dev):
        self.device_path = dev
        try:
            self.serial_connection = serial.Serial(self.device_path, self.config.device.baudrate)
            self.serial_connection.timeout = self.config.device.timeout
        except serial.serialutil.SerialException as e:
            self.abort('Unable to open serial connection to device: %s\nError: %s' % (dev, e.strerror), close_serial=False)


    def setup_logger(self):
        logger = logging.getLogger()
        # TODO: Make the log file more configurable
        fileHandler = logging.FileHandler("kubos_linux_test.log")
        logger.addHandler(fileHandler)
        consoleHandler = logging.StreamHandler()
        logger.addHandler(consoleHandler)


    def load_configuration(self, config_file):
        self.config = kubos_config.KubosTestConfig()
        self.config.load_config(config_file)


    def remove_non_specified_tests(self, specified_tests):
        new_list = []
        for test in self.config.tests:
            if test.name in specified_tests:
                self.logger.info('Leaving in test "%s" as it was specified' % test.name)
                new_list.append(test)
            else:
                self.logger.info('Excluding test "%s" in' % test.name)
        self.config.tests = new_list


    def run_tests(self):
        if self.config.login.username:
            self.login()
        for test in self.config.tests:
            self.run_test(test)
        self.print_test_summary()


    def run_test(self, test):
        self.logger.info("Running test: %s" % test.name)

        if test.pre_test:
            self.logger.info("Test: %s Running pre-test command: %s\n" % (test.name, test.pre_test))
            self.run_cmd(test.pre_test)

        # Build the project
        if test.build_source:
            resource_type = self.get_resource_type(test.build_source)

            if resource_type == 'url':
                proj_dir = self.clone_repo(test.build_source)
            elif resource_type == 'path':
                proj_dir = test.build_source
            self.build_project(proj_dir)
            # Flash the project
            if not test.flash_source:
                test.flash_source = proj_dir

        if test.flash_source:
            self.flash_project(test.flash_source)

        # Run the test command
        output = self.run_serial_command(test.test_command)
        self.verify_test_output(test, output)

        if test.post_test:
            self.logger.info("Test: %s Running post-test command: %s\n" % (test.name, test.post_test))
            self.run_cmd(test.post_test)


    def build_project(self, src):
        self.logger.info('Building Project: %s' % src)
        start_dir = os.getcwd()

        if not os.path.isabs(src):
            src = self.get_abs_path(start_dir, src)
        os.chdir(src)

        self.run_cmd('kubos', 'clean')
        self.run_cmd('kubos', 'link', '--all')
        ret_code = self.run_cmd('kubos', '-t', self.config.device.target, 'build')

        if ret_code != 0:
            self.abort('Building project %s resulted in a non-zero exit code: %d.' % (proj_dir, ret_code))

        os.chdir(start_dir)


    def flash_project(self, proj):
        self.logger.info('Flashing Project: %s' % proj)
        start_dir = os.getcwd()
        flash_args = ['kubos', '-t', self.config.device.target, 'flash']

        if not os.path.isabs(proj):
            proj = self.get_abs_path(start_dir, proj)

        if os.path.isfile(proj):
            # Flash the project as a standalone file
            flash_args.append(proj)
            proj_dir = self.get_flash_project_dir()
        elif os.path.isdir(proj):
            # Flash it like a regular kubos project - no additional args are needed
            proj_dir = proj
        else:
            abort('Unable to flash unknown type of resource: %s' % proj)

        os.chdir(proj_dir)
        os.environ["PWD"] = proj_dir # The flash script depends on this environment variable

        ret_code = self.run_cmd(*flash_args, cwd=proj_dir)

        if ret_code != 0:
            self.abort('Flashing project %s resulted in a non-zero exit code, output: %d.' % (proj_dir, ret_code))

        # Flashing a file to a linux board logs us out..
        self.login()
        os.chdir(start_dir)


    def run_serial_command(self, command):
        '''
        sends a command string (command) over the serial device and reads the
        output until the next prompt (self.serial.promt) is read.
        '''
        self.serial_connection.write('%s\n' % str(command))
        output = self.serial_connection.read(self.MAX_SERIAL_READ_LEN).replace('\r', '')
        # Parse the output
        command_len = len(command) + 1
        prompt_len = len(self.config.device.prompt) + 1
        cmd_output = output[command_len : -prompt_len]
        return cmd_output


    def verify_test_output(self, test_data, actual):
        passed = False
        if test_data.expected_is_regex:
            expected_regex = re.compile(test_data.expected_test_output)
            if expected_regex.match(actual):
                passed = True
        else:
            if actual == test_data.expected_test_output:
                passed = True

        if passed:
            self.add_test_success("Test %s passed" % test_data.name)
        else:
            if test_data.abort_on_failure:
                self.abort('Test: %s Failed:\n Expected:\n"%s"\n\n Did not match actual:\n"%s"' % (test_data.name, test_data.expected_test_output, actual))
            else:
                self.add_test_failure('Test: %s Failed:\n Expected:\n"%s"\n\n Did not match actual:\n"%s"' % (test_data.name, test_data.expected_test_output, actual))


    def add_test_success(self, message):
        self.logger.info(message)
        self.test_summary.append('%s%s%s' % (self.GREEN, message, self.NORMAL))


    def add_test_failure(self, message):
        self.logger.error(message)
        self.test_summary.append('%s%s%s' % (self.RED, message, self.NORMAL))


    def login(self):
        self.logger.info('logging in')
        old_timeout = self.serial_connection.timeout
        self.serial_connection.timeout = 1
        self.serial_connection.flush()
        self.serial_connection.reset_input_buffer()
        self.serial_connection.write('\n\n')
        data = self.serial_connection.read(self.MAX_SERIAL_READ_LEN)
        if self.config.device.prompt in data:
            # already logged in
            pass
        elif "login:" in data:
            self.serial_connection.write('%s\n' % str(self.config.login.username))
            time.sleep(3)
            self.serial_connection.write('%s\n' % str(self.config.login.password))
            time.sleep(2)
            self.serial_connection.read(self.MAX_SERIAL_READ_LEN)
        else:
            self.logger.info('executable may own terminal..attempting to close')
            self.serial_connection.write('\x03')
            self.serial_connection.write('\n\n')
            data = self.serial_connection.read(self.MAX_SERIAL_READ_LEN)
            if self.config.device.prompt not in data:
                self.logger.error('debug terminal may not be accessible')
        self.serial_connection.timeout = old_timeout


    def print_test_summary(self):
        self.logger.info('\n\nTest Summary:\n\n')
        for line in self.test_summary:
            self.logger.info(line)
        self.close_serial()


    def build_flash_proj(self, proxy_proj_path):
        '''
        With the current limitations of the CLI flashing and the requirements,
        the hello-world project is used as the "proxy project" for flashing.

        An additional requirement of having a ../build/<target_name> directory
        is satisfied by building this project for the current target
        '''
        self.logger.info('Building the linux flash proxy project')
        self.build_project(proxy_proj_path)


    def get_resource_type(self, obj):
        if urlparse.urlparse(obj).scheme != "":
            return 'url'
        elif os.path.exists(obj):
            return 'path'
        else:
            self.abort('unknown type of entity %s. This should be a path (relative or absolute) or a URL' % obj)


    def abort(self, message, close_serial=True):
        self.logger.error(message)
        if close_serial:
            self.close_serial()
        sys.exit(1)


    def close_serial(self):
        self.serial_connection.close()


def main():
    parser = argparse.ArgumentParser(description='Integration Test Runner')
    parser.add_argument('config_file', help='The path to the test specific config file you want to test.')
    parser.add_argument('device_path', nargs='?', default='/dev/FTDI', help='The path to your serial device')
    parser.add_argument('--tests', nargs='*', default=None, help='A list of tests to run. If provided only the listed tests will be run.')

    args = parser.parse_args()
    runner = TestRunner(args.config_file, args.tests)
    runner.setup_serial_connection(args.device_path)
    runner.run_tests()


if __name__ == "__main__":
    main()

