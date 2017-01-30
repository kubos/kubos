#!/usr/bin/env python
from __future__ import print_function
import argparse
import json
import os
import subprocess
import sys
from kubos_build import KubosBuild

this_dir = os.path.abspath(os.path.dirname(__file__))
root_dir = os.path.dirname(this_dir)

class KubosBuilder(object):
    def __init__(self):
        self.kb = KubosBuild()
        self.modules = self.kb.modules()
        self.targets = self.kb.targets()

    def cmd(self, *args, **kwargs):
        cwd = kwargs.get('cwd', os.getcwd())
        save_output = kwargs.pop('save_output', False)
        echo = kwargs.pop('echo', False)

        if echo:
            print(' '.join(args))
        try:
            if save_output:
                return subprocess.check_output(args, **kwargs)
            else:
                return subprocess.check_call(args, stdout=subprocess.PIPE, stderr=subprocess.PIPE, **kwargs)
        except subprocess.CalledProcessError, e:
            #print('Error executing command, giving up',file=sys.stderr)
            return 1

    def build(self, module_name="", target_name=""):
        #module_list = next((m for m in self.kb.modules() if m.yotta_name() == module_name), None)
        module = next((m for m in self.kb.modules() if m.yotta_name() == module_name), None)
        target = next((t for t in self.kb.targets() if t.yotta_name() == target_name), None)
        if module and target:
            print('Building [module %s@%s] for [target %s] - ' % (module.yotta_name(), module.path, target_name), end="")
            self.cmd('kubos', 'target', target_name, cwd=module.path)
            self.cmd('kubos', 'clean', cwd=module.path)
            ret = self.cmd('yt', 'build', cwd=module.path)
            #print('Passed' if ret is 0 else 'Failed')
            print('Result %d' % ret)
            return ret
        else:
            if module is None:
                print("Module %s was not found" % module_name)
            if target is None:
                print("Target %s was not found" % target_name)
            return 1

    def list_targets(self):
        for target in self.kb.targets():
            print(target.yotta_name())

    def list_modules(self):
        for module in self.kb.modules():
            print(module.yotta_name())

    def find_modules(self, path):
        path_list = path.split("/")
        modules = set()
        # Pop off file name for first directory
        path_list.pop()
        while len(path_list):
            new_path = "" + "/".join(path_list)
            kubos_build = KubosBuild(kubos_dir=new_path)
            for p in kubos_build.projects:
                if p.type != "unknown":
                    modules.add(p.yotta_name())
            if len(modules):
                break

            path_list.pop()
        return modules

    def list_changed_modules(self):
        try:
            git_output = subprocess.check_output(["git", "diff", "--numstat", "HEAD^!"])
            git_lines = [l for l in git_output.splitlines()]
            file_paths = [l.split()[2] for l in git_lines]
            modules = set()
            for path in file_paths:
                modules = modules | (self.find_modules(path))
            
            if len(modules):
                print("Modules changed in last commit:")
            for m in modules:
                print(m)
        except subprocess.CalledProcessError:
            print("Error getting changed modules")

    def build_all_targets(self, module_name=""):
        module = next((m for m in self.kb.modules() if m.yotta_name() == module_name), None)
        if module:
            for target in self.kb.targets():
                self.build(module.yotta_name(), target.yotta_name())

    def build_all_modules(self, target_name=""):
        target = next((t for t in self.kb.targets() if t.yotta_name() == target_name), None)
        if target:
            for module in self.kb.modules():
                self.build(module.yotta_name(), target.yotta_name())
    
    def build_all_combinations(self):
        for target in self.kb.targets():
            for module in self.kb.modules():
                self.build(module.yotta_name(), target.yotta_name())

def main():
    parser = argparse.ArgumentParser(
        description='Builds KubOS modules')

    parser.add_argument('--target', metavar='target',
                        help='Specifies target to build modules for')
    parser.add_argument('--module', metavar='module',
                        help='Specifies modules to build')
    parser.add_argument('--all_targets', action='store_true', default=False,
                        help='Builds module for all targets')
    parser.add_argument('--all_modules', action='store_true', default=False,
                        help='Builds all modules for target')
    parser.add_argument('--list_targets', action='store_true', default=False)
    parser.add_argument('--list_modules', action='store_true', default=False)
    parser.add_argument('--list_changed_modules', action='store_true', default=False)
        

    args = parser.parse_args()

    builder = KubosBuilder()

    ret = 0

    if args.list_targets:
        builder.list_targets()
    if args.list_modules:
        builder.list_modules()
    if args.list_changed_modules:
        builder.list_changed_modules()
    if args.target and args.module:
        ret = builder.build(module_name=args.module, target_name=args.target)
    elif args.module and args.all_targets:
        builder.build_all_targets(module_name=args.module)
    elif args.target and args.all_modules:
        builder.build_all_modules(target_name=args.target)
    elif args.all_targets and args.all_modules:
        builder.build_all_combinations()
    else:
        ret = -1

    sys.exit(ret)

if __name__ == '__main__':
    main()
