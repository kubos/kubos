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

import json
import os

module_json = 'module.json'
# project_file = os.path.join(this_dir, 'projects.json')


class KubosUtils(object):
    def __init__(self):
        self.ignore_list = ['.git']
        self.search_depth = 3
        self.this_dir = os.path.dirname(__file__)
        self.kubos_root = os.path.abspath(os.path.join(self.this_dir, '..'))
        self.module_index = {}
        self.discover_kubos_modules()


    def discover_modules_rec(self, path, depth=1):
        for thing in os.listdir(path):
            if depth > self.search_depth:
                return
            thing_path = os.path.join(path, thing)
            if thing == module_json:
                module_name = self.get_module_name(thing_path)
                if module_name is not None:
                    self.module_index[module_name] = path
                return
            elif os.path.isdir(thing_path):
                self.discover_modules_rec(thing_path, depth=depth+1)


    def get_module_name(self, path):
        if not os.path.isfile(path):
            return None
        with open(path, 'r') as _file:
            data = json.loads(_file.read())
        if 'name' in data:
            return data['name']
        else:
            return None


    def discover_kubos_modules(self):
        self.discover_modules_rec(self.kubos_root)


if __name__ == '__main__':
    util = KubosUtils()
