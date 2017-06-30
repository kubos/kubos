#!/usr/bin/env python2
#
# KubOS Core Flight Services
# Copyright (C) 2015 Kubos Corporation
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
#
# A convenience script for building and running RIOT executables with docker
#
import argparse
import glob
import os
import shlex
import sys
import subprocess

this_dir = os.path.dirname(os.path.abspath(__file__))
kubos_root = os.path.dirname(this_dir)
pwd = os.environ['PWD']
cmd_relpath = os.path.relpath(pwd, kubos_root)

def machine_config(machine):
    cfg = subprocess.check_output('docker-machine config ' + machine,
                                  shell=True)
    if not cfg:
        return []

    return shlex.split(cfg.strip())

def find_elf():
    binaries = glob.glob('./bin/native/*.elf')
    if len(binaries) == 0:
        return None

    return os.path.relpath(binaries[0], pwd)

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('-m', '--machine', help='Docker machine config to use')
    parser.add_argument('command', help='make|run')
    args, extra = parser.parse_known_args()

    docker_cmd = ['docker']
    if args.machine:
        docker_cmd.extend(machine_config(args.machine))

    docker_cmd.extend(['run', '-it', '-v', kubos_root + ':/data/riotbuild',
                       '-w', '/data/riotbuild/' + cmd_relpath, 'riotbuild'])

    if args.command == 'make':
        docker_cmd.append('make')
        docker_cmd.extend(extra)
    elif args.command == 'run':
        elf_relpath = find_elf()
        if not elf_relpath:
            parser.error('No ELF binaries found in ./bin/native')

        docker_cmd.append(elf_relpath)
        docker_cmd.extend(extra)
    else:
        parser.error('Unknown command: "%s"' % args.command)

    print '>', ' '.join(docker_cmd)
    os.execvp('docker', docker_cmd)

if __name__ == '__main__':
    main()
