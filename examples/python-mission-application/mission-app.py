#!/usr/bin/env python

# Copyright 2018 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.

"""
Example Python mission application showing the different run levels
available and how to interact with onboard services.

This application is meant to be run through the mission application
service, but can be run directly from the command line as well.

NOTE: Mission application service will NOT run Python mission apps
without the environment indicator at the top of the file:
"#!/usr/bin/env python"
"""

import argparse
import toml
import time
import app_api


MANIFEST = toml.load('manifest.toml')
SERVICES = app_api.Services()
LOGFILE = '/home/system/var/app-log'


def on_boot():

    with open(LOGFILE, 'a+') as log:
        log.write("OnBoot logic\n")
    query = '{ apps { active, app { uuid, name, version, author } } }'
    while True:
        try:
            with open(LOGFILE, 'a+') as log:
                log.write("###############################################\n")
                log.write("I'm performing recurring system housekeeping\n")
                log.write("Querying for active applications!\n")
                log.write("Query: {}\n".format(query))
                apps = SERVICES.query(service="app-service", query=query)
                log.write("Active applications are: {}\n".format(apps))
        except Exception as e:
            with open(LOGFILE, 'a+') as log:
                log.write("Housekeeping caused an error: {},{},{}\n".format(
                    type(e), e.args, e))

        with open(LOGFILE, 'a+') as log:
            log.write("\n\n")
        time.sleep(10)


def on_command(cmd_args):

    with open(LOGFILE, 'a+') as log:
        log.write("OnCommand logic\n")

    if cmd_args.cmd_string == 'safemode':
        if cmd_args.cmd_int > 0:
            with open(LOGFILE, 'a+') as log:
                log.write(
                    "Going into safemode for {} seconds\n".format(
                        cmd_args.cmd_int))
                log.write("Sending commands to hardware to go into safemode\n")
                time.sleep(cmd_args.cmd_int)
                log.write("Sending commands to hardware to normal operation\n")
        else:
            raise ValueError("Command Integer must be positive and non-zero\n")
    else:
        # If no command args are given, just print manifest info
        with open(LOGFILE, 'a+') as log:
            log.write("My manifest information: {}\n".format(MANIFEST))


def main():
    parser = argparse.ArgumentParser()

    parser.add_argument(
        '-r',
        '--run',
        nargs=1,
        default='OnBoot',
        help='Determines run behavior. Either "OnBoot" or "OnCommand"',
        required=True)
    parser.add_argument(
        '-s',
        '--cmd_string',
        help='Command Argument String passed into OnCommand behavior',
        required=False)
    parser.add_argument(
        '-i',
        '--cmd_int',
        type=int,
        help='Command Argument Integer passed into OnCommand behavior',
        required=False)

    args = parser.parse_args()

    if args.run[0] == 'OnBoot':
        on_boot()
    else:
        on_command(args)


if __name__ == "__main__":
    main()
