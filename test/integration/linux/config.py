import os
import json
import sys

class KubosTestConfig(object):
    tests = []

    class Device(object):
        DEFAULT_BAUDRATE = 115200
        DEFAULT_TIMEOUT  = 10

        def __init__(self):
            self.baudrate = self.DEFAULT_BAUDRATE
            self.prompt = None
            self.target = None
            self.timeout = self.DEFAULT_TIMEOUT


    class Login(object):
        DEFAULT_USERNAME = 'root'
        DEFAULT_PASSWORD = 'Kubos123'

        def __init__(self):
            self.username = self.DEFAULT_USERNAME
            self.password = self.DEFAULT_PASSWORD


    class Test(object):
        def __init__(self):
            self.name = None
            self.abort_on_failure = False
            self.pre_test  = None
            self.build_source = None
            self.flash_source = None
            self.post_test = None
            self.expected_is_regex = None
            self.expected_test_output = None


    def __init__(self):
        self.device = self.Device()
        self.login = self.Login()
        self.tests = []


    def load_config(self, config_file):
        if not os.path.isfile(config_file):
            # TODO: clean this up
            print >>sys.stderr, 'The config file %s does not exist. Aborting.' % config_file
            sys.exit(1)

        with open(config_file, 'r') as _file:
            data = json.loads(_file.read())
        self.load_device(data)
        self.load_login(data)
        self.load_tests(data)
        return self


    def load_device(self, data):
        self.load_simple_section(self.device, data['device'])


    def load_login(self, data):
        self.load_simple_section(self.login, data['login'])


    def load_tests(self, data):
        for test_data in data['tests']:
            temp_test = self.Test()
            self.load_simple_section(temp_test, test_data)
            self.tests.append(temp_test)


    def load_simple_section(self, attr, data):
        for key in data:
            setattr(attr, key, data[key])

