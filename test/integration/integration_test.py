#!/usr/bin/python

import json
import kubos
import logging
import mock
import os
import sys
import unittest

from sets import Set
from kubos.test.utils import KubosTestCase
from kubos.utils.constants import GLOBAL_TARGET_PATH

class CLIIntegrationTest(KubosTestCase):
    default_branch = 'master'
    branch_key = 'CIRCLE_BRANCH' #Run the tests against the branch this Circle test was triggered by

    def setUp(self):
        super(CLIIntegrationTest, self).setUp()
        self.proj_name = 'project-test'
        self.proj_dir = os.path.join(self.base_dir, self.proj_name)
        self.target_list = None #Defined at test runtime
        self.branch_name = self.get_branch_name()
        self.first_arg = sys.argv[0]
        sys.argv = [self.first_arg] #clear any excess command line arguhements
        logging.error = mock.MagicMock()


    def get_branch_name(self):
        return os.environ[self.branch_key] if self.branch_key in os.environ else self.default_branch


        #The tests are run in alphabetical order by default (hence the 1,2,3,etc..)
        #TODO: Get a better way of running the test in order while still being readable
    def test_1_update_source_check_versions(self):
        self.run_command('update')
        os.chdir(self.base_dir)
        self.run_command('version')
        self.run_command('versions')
        print "Branch: %s" % self.branch_name
        self.run_command('use', '--branch', self.branch_name)
        self.run_command('version')
        self.run_command('versions')


    def test_2_rt_example(self):
        self.run_command('init', self.proj_name)
        os.chdir(self.proj_dir)
        self.run_command('target')
        rt_list, linux_list = self.get_target_lists()
        for target in rt_list:
            self.run_command('target', target)
            self.run_command('target') #print the target
            self.run_command('list')
            self.run_command('build')
            self.run_command('clean')
        os.chdir(self.base_dir)


    def test_3_linux_example(self):
        self.run_command('init', '-l', self.proj_name)
        os.chdir(self.proj_dir)
        self.run_command('target')
        rt_list, linux_list = self.get_target_lists()
        for target in linux_list:
            self.run_command('target', target)
            self.run_command('target')
            self.run_command('list')
            self.run_command('build')
            self.run_command('clean')
        os.chdir(self.base_dir)


    def run_command(self, subcommand_name, *args):
        arg_list = [subcommand_name] + list(args)
        #store the current command line arguments so we can restore them later
        starting_args = sys.argv
        #set up new command line args
        sys.argv = sys.argv + arg_list
        print '\nRunning command %s %s' % (subcommand_name, ' '.join(args))

        #run the command
        return_code = kubos.main()
        self.assertEqual(return_code, 0)
        logging.error.assert_not_called() #secondary safeguard for detecting runtime errorsa
        # logging.error.reset()

        #reset the command line arguments that we added during this command run
        sys.argv = starting_args


    def get_target_lists(self):
        '''
        Splits rt and linux targets.
        Returns the list of kubos_rt targets and the list of linux targets.
        '''
        linux_list = []
        rt_list = []
        target_list = self.get_all_eligible_targets()

        #TODO: Get a better way of determining linux targets
        for target in target_list:
            if 'linux' in target:
                linux_list.append(target)
            else:
                rt_list.append(target)
        return rt_list, linux_list


    def get_all_eligible_targets(self):
        '''
        Returns the list of targets which do not have dependent targets.
        Example target hierarchy:
        kubos-gcc
          |____kubos-rt-gcc
                 |____kubos-arm-none-eabi-gcc
                        |____stm32f407-disco-gcc <- This is the only target we want to build
        The other targets in the hierarchy are not meant to be built against
        '''
        inherit_key = 'inherits'
        name_key    = 'name'
        ineligible_set = Set()
        complete_set   = Set()
        target_dir_list = os.listdir(GLOBAL_TARGET_PATH)

        for subdir in target_dir_list:
            json_data = self.get_target_json_data(subdir)
            if name_key in json_data:
                complete_set.add(json_data['name'])
            if inherit_key in json_data:
                #The target this current target depends on is an ineligible target
                target_dependency = json_data[inherit_key].keys()
                ineligible_set.add(*target_dependency)
        return complete_set - ineligible_set



    def get_target_json_data(self, subdir):
        target_json = os.path.join(GLOBAL_TARGET_PATH, subdir, 'target.json')
        if os.path.isfile(target_json):
            with open(target_json, 'r') as target_file:
                json_data = json.loads(target_file.read())
                return json_data
        return None


    def tearDown(self):
        super(CLIIntegrationTest, self).tearDown()


if __name__ == '__main__':
    unittest.main()
