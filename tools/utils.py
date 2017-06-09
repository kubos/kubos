import subprocess
import os
import sys

import json
import os

module_json = 'module.json'

class KubosUtils(object):
    def __init__(self):
        self.ignore_list = ['.git']
        self.search_depth = 3
        self.this_dir = os.path.dirname(__file__)
        self.kubos_root = os.path.abspath(os.path.join(self.this_dir, '..'))
        self.module_index = {}

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
        return self.module_index


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


def get_kubos_modules():
    '''
    Returns a dictionary of the Kubos module names and their absolute paths
    '''
    kubos_util = KubosUtils()
    return kubos_util.discover_kubos_modules


def cmd(*args, **kwargs):
    '''
    Maintains compatibility with existing python utilities.
    '''
    kubos_util = KubosUtils()
    return kubos_util.run_cmd(*args, **kwargs)


if __name__ == '__main__':
    util = KubosUtils()

