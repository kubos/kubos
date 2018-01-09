#!/usr/bin/python
import argparse
import docker
import json
import os
import sys
import re

from .. import utils

'''
This is a utility for checking the versions of utilities in a docker image (kubos/kubos-dev)
against a vagrant environment. The path to the environment needs to be provided as an argument to
this script.

It reads in programs to test from the programs.json file. Add or remove programs from that list as
necessary.

Usage:
$ cd <kubos repo root>
$ python -m tools.dist.version_check <path-to-vagrant-environment>
'''


class VersionCheck(object):
    docker_image = 'kubos/kubos-dev:latest'
    json_file = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'programs.json')
    client = docker.from_env()
    pattern = re.compile(r'\s+')

    def __init__(self, vagrant_path):
        self.vagrantfile = vagrant_path
        with open(self.json_file, 'r') as json_file:
            self.programs = json.loads(json_file.read())['programs']
        os.chdir(vagrant_path)


    def check_versions(self):
        print 'Starting version checking...'
        for prog in self.programs:
            self.check_version(prog)


    def check_version(self, program):
        #these are the easy to read versions
        vagrant_version = self.check_vagrant_version(program)
        docker_version  = self.check_docker_version(program)
        #differences in whitespace screw up the comparison
        comp_docker_version  = re.sub(self.pattern, '', docker_version)
        comp_vagrant_version = re.sub(self.pattern, '', vagrant_version)
        if comp_vagrant_version == comp_docker_version:
            print 'Found Matching version: %s' % program
        else:
            print 'FAILED: Program %s, Vagrant version: %s Docker version: %s' % (program, vagrant_version, docker_version)


    def check_vagrant_version(self, program):
        FDNULL = open(os.devnull, 'w')
        return utils.cmd('vagrant', 'ssh', '-c', '%s --version' % program, save_output=True, echo=False, stderr=FDNULL)


    def check_docker_version(self, program):
        return self.client.containers.run('kubos/kubos-dev', '%s --version' % program)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('vagrant_file', help='Specify the path to the Vagrant environment you want to sanity check')
    args, unknown_args = parser.parse_known_args()

    checker = VersionCheck(args.vagrant_file)
    checker.check_versions()


if __name__ == '__main__':
    main()
