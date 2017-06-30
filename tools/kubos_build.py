#!/usr/bin/env python2
# common code for dealing with KubOS repo/git and yotta metadata
import json
import os
import subprocess
import sys
import glob
import utils

this_dir = os.path.abspath(os.path.dirname(__file__))
kubos_dir = os.path.dirname(this_dir)

class Project(object):
    def __init__(self, name, path):
        self.name = name
        self.path = path
        self.type = 'unknown'
        self.yotta_data = None

        if os.path.isfile(path + '/module.json'):
            self.yotta_data = json.load(open(path + '/module.json', 'r'))
            self.type = 'yotta_module'
        elif os.path.isfile(path + '/target.json'):
            self.yotta_data = json.load(open(path + '/target.json', 'r'))
            self.type = 'yotta_target'

        self.version = self.get_version()

    def is_bin(self):
        if not self.yotta_data:
            return False
        return 'bin' in self.yotta_data

    def yotta_name(self):
        if not self.yotta_data:
            return None
        return self.yotta_data['name']

    def get_commit_sha(self):
        try:
            sha = subprocess.check_output(['git', 'rev-parse', '@'],
                                          cwd=self.path)
            return sha.strip()
        except subprocess.CalledProcessError, e:
            return None

    def get_last_tag(self):
        try:
            tag_list = subprocess.check_output(['git', 'show-ref', '--tags'],
                                               cwd=self.path)
            last_tag = tag_list.splitlines()[-1]
            return last_tag.split(' ')[1].strip()
        except:
            return ''

    def get_version(self):
        try:
            return self.yotta_data['version']
        except:
            return 'No data'

    def find_upstream_branch(self):
        try:
            FNULL = open(os.devnull, 'w')
            upstream = subprocess.check_output(['git', 'rev-parse',
                                                '--abbrev-ref',
                                                '--symbolic-full-name', '@{u}'],
                                               cwd=self.path, stderr=FNULL,
                                               close_fds=True)


            remote, branch = upstream.strip().split('/')
            remote_url = subprocess.check_output(['git', 'config', '--get',
                                                 'remote.%s.url' % remote],
                                                 cwd=self.path)

            return dict(remote=remote, branch=branch, url=remote_url.strip())
        except subprocess.CalledProcessError, e:
            return None


class KubosBuild(object):
    def __init__(self, kubos_dir=kubos_dir):
        self.kubos_dir = kubos_dir
        self.find_projects()

    def modules(self, include_bin=True):
        mod_filter = lambda c: c.type == 'yotta_module' and (include_bin or not c.is_bin())
        return filter(mod_filter, self.projects)

    def bin_modules(self):
        return filter(lambda c: c.type == 'yotta_module' and c.is_bin(), self.projects)

    def targets(self):
        return filter(lambda c: c.type == 'yotta_target', self.projects)
    
    def build_targets(self):
        return filter(lambda c: 'buildTarget' in c.yotta_data, self.targets())

    def find_projects(self):
        self.projects = []
        try:
            modules = subprocess.check_output(["find", ".", "-name",
                                            "module.json"], cwd=self.kubos_dir)
            for path in modules.splitlines():
                if "yotta_modules" not in path:
                    path = path.replace("module.json", "").strip()
                    name = path.split("/")[-2]
                    path = os.path.abspath(self.kubos_dir + "/" + path)
                    self.projects.append(Project(name, path))

            modules = subprocess.check_output(["find", ".", "-name",
                                            "target.json"], cwd=self.kubos_dir)
            for path in modules.splitlines():
                if "yotta_targets" not in path:
                    path = path.replace("target.json", "").strip()
                    name = path.split("/")[-2]
                    path = os.path.abspath(self.kubos_dir + "/" + path)
                    self.projects.append(Project(name, path))
        except OSError:
            pass

