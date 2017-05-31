#!/usr/bin/python

# Copyright (C) 2017 Kubos Corporation
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

'''
Script for running scan-build and aggregating project output

Run from kubos repo root directory:
$ python -m tools.ci.lint
'''

import json
import os
import sys
from .. import utils
import subprocess

class KubosLintTest(object):
    GREEN  = '\033[92m'
    RED    = '\033[91m'
    NORMAL = '\033[0m'
    base_dir = os.path.dirname(os.path.abspath(__file__))
    project_file = os.path.join(base_dir, 'lint-projects.json')
    lint_output_dir = os.environ['CIRCLE_ARTIFACTS'] if 'CIRCLE_ARTIFACTS' in os.environ else base_dir

    def __init__(self):
        self.utils = utils.KubosUtils()
        self.default_target = 'x86-linux-native'
        self.module_index = self.utils.discover_kubos_modules()
        self.lint_projects = self.load_lint_projects()
        self.check_deps()


    def check_deps(self):
        '''
        This sets up our docker environment
        '''
        kubos_dir = os.path.join(os.path.expanduser('~'), '.kubos')
        if not os.path.isdir(kubos_dir):
            self.utils.run_cmd('kubos', 'update')
        self.utils.run_cmd('kubos', 'use', '-b', os.environ['CIRCLE_BRANCH'])


    def load_lint_projects(self):
        if os.path.isfile(self.project_file):
            with open(self.project_file, 'r') as _file:
                data = json.loads(_file.read())
            return data['lint-projects']
        else:
            print 'The lint-projects.json file was not found. Unable to continue the static analysis.'
            sys.exit(1)


    def run_lint_tests(self):
        failed_projects = []
        passed_projects = []
        for proj in self.lint_projects:
            if proj in self.module_index:
                proj_dir = self.module_index[proj]
                ret_code = self.lint_proj(proj, proj_dir)
                if ret_code == 0:
                    passed_projects.append(proj)
                else:
                    failed_projects.append(proj)
            else:
                print 'Unable to find project: %s' % proj
                failed_projects.append('%s - Not Found' % proj)
        #Print the successful projects
        if len(passed_projects) != 0:
            print 'Successful project builds:'
            for project in passed_projects:
                print self.GREEN + 'Passed: %s' % project + self.NORMAL

        if len(failed_projects) != 0:
            print 'Failed project builds:'
            for project in failed_projects:
                print self.RED + project + self.NORMAL
            sys.exit(1)
        passed_len = len(passed_projects)
        failed_len = len(failed_projects)
        total_len  = passed_len + failed_len
        print '\nSummary: Total %s projects attempted. %s projects passed. %s projects failed.' % (total_len, passed_len, failed_len)


    def lint_proj(self, proj, proj_dir):
        build_dir = os.path.join(proj_dir, 'build')
        output_dir = os.path.join(self.lint_output_dir, proj + '-lint-output')
        if os.path.isdir(build_dir):
            self.utils.run_cmd('kubos', 'clean', cwd=proj_dir) #If the project's built we need to clean and rebuild it
        self.utils.run_cmd('kubos', 'link', '-a', cwd=proj_dir)
        #scan build tinkers with the build config some so we need to rebuild the project from scratch
        return self.utils.run_cmd('scan-build', '-o', output_dir, 'kubos', '--target', self.default_target, 'build', cwd=proj_dir, echo=True)


if __name__ == '__main__':
    lint = KubosLintTest()
    lint.run_lint_tests()

