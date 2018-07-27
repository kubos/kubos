#!/usr/bin/env python

# Copyright 2018 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.

"""
Example Python mission application showing the different run levels
available and how to interact with onboard services.

This application is meant to be run through the mission application
service, but can be run directly from the command line as well.
"""

import argparse
import toml
import time
import app_api

MANIFEST = toml.load('manifest.toml')
SERVICES = app_api.Services()


def on_boot():

    print "OnBoot logic\n"
    query = '{ apps { active, app { uuid, name, version, author } } }'
    while True:
        try:
            print("##################################################")
            print("I'm performing recurring system housekeeping logic")
            print("Querying the application service for active applications!")
            print("Query: {}".format(query))
            apps = SERVICES.query(service="app-service", query=query)
            print("Active applications are: {}".format(apps))
        except Exception as e:
            print("Housekeeping caused an error: {},{},{}".format(
                type(e), e.args, e))

        print("\n\n")
        time.sleep(10)


def on_command(cmd_args):

    print "OnCommand logic"

    if cmd_args.cmd_string == 'safemode':
        if cmd_args.cmd_int > 0:
            print(
                "I'm been commanded to go into safemode for {} minutes".format(
                    cmd_args.cmd_int))
            print("send commands to hardware to go into safemode")
            time.sleep(cmd_args.cmd_int * 60)
            print("send commands to hardware to resume normal operation")
        else:
            raise ValueError("Command Integer must be positive and non-zero")
    elif cmd_args.cmd_string == 'manifest':
        print "My manifest information: {}".format(MANIFEST)
    else:
        raise ValueError(
            'Command "{}" not supported!'.format(cmd_args.cmd_string))


def main():
    parser = argparse.ArgumentParser()

    parser.add_argument('--run', '-r', nargs=1, default='OnBoot')
    parser.add_argument(
        '--cmd_string',
        '-s',
        help='Command Argument String passed into OnCommand behavior')
    parser.add_argument(
        '--cmd_int', '-i', type=int,
        help='Command Argument Integer passed into OnCommand behavior')

    args = parser.parse_args()

    if args.run[0] == 'OnBoot':
        on_boot()
    else:
        on_command(args)


if __name__ == "__main__":
    main()
