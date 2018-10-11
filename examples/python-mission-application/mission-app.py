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
import app_api
import datetime
import toml
import time

SERVICES = app_api.Services()
BOOTFILE = '/home/system/var/onboot-output'
COMMANDFILE = '/home/system/var/oncommand-output'
ERRORSFILE = '/home/system/var/mission-errors'

# Helper function to insert a timestamp into the log message
def write_log(logfile, message):

    logfile.write("%s: %s\n" % (str(datetime.datetime.now()), message))

# On-boot logic which will be called at boottime if this app is registered with
# the applications service
def on_boot():

    with open(BOOTFILE, 'a+') as file:
        write_log(file, "OnBoot logic")

    while True:
        # Get the current amount of available memory from the monitor service
        try:
            request = '{memInfo{available}}'
            response = SERVICES.query(service="monitor-service", query=request)
        except Exception as e: 
            write_log(file, "Something went wrong: " + str(e) + "")
            continue
        
        data = response["memInfo"]
        available = data["available"]
        
        write_log(file, "%s: Current available memory: %s kB" % (str(datetime.datetime.now()), available))
        
        request = '''
            mutation {
                insert(subsystem: "OBC", parameter: "available_mem", value: "%s") {
                    success,
                    errors
                }
            }
            ''' % (available)
        
        # Save the result to the telemetry database
        try:
            response = SERVICES.query(service="telemetry-service", query=request)
        except Exception as e: 
            write_log(file, "Something went wrong: " + str(e) + "")
            print "OnCommand logic encountered errors"
            continue
            
        data = response["insert"]
        success = data["success"]
        errors = data["errors"]
        
        if success == False:
            write_log(file, "Telemetry insert encountered errors: " + str(errors) + "")

        with open(BOOTFILE, 'a+') as file:
            write_log(file, "\n")
        
        # Wait five minutes before checking again
        time.sleep(300)

# On-demand logic which will be called manually by the user (potentially via the applications service)
def on_command(cmd_args):

    with open(COMMANDFILE, 'a+') as file:
        write_log(file, "OnCommand logic")

    if cmd_args.cmd_string == 'safemode':
        if cmd_args.cmd_int > 0:
            with open(LOGFILE, 'a+') as file:
                write_log(file, 
                    "Going into safemode for {} seconds".format(
                        cmd_args.cmd_int))
                write_log(file, "Sending commands to hardware to go into safemode")
                time.sleep(cmd_args.cmd_int)
                write_log(file, "Sending commands to hardware to normal operation")
        else:
            raise ValueError("Command Integer must be positive and non-zero")
                    
    else:
        query = '{ apps { active, app { uuid, name, version, author } } }'
        try:
            with open(COMMANDFILE, 'a+') as file:
                write_log(file, "Querying for active applications")
                write_log(file, "Query: {}".format(query))
                apps = SERVICES.query(service="app-service", query=query)
                write_log(file, "Active applications are: {}".format(apps))
        except Exception as e:
            with open(COMMANDFILE, 'a+') as file:
                write_log(file, "Housekeeping caused an error: {},{},{}".format(
                    type(e), e.args, e))
        

def main():
    parser = argparse.ArgumentParser()

    # The -r argument is required to be present by the applications service
    parser.add_argument(
        '-r',
        '--run',
        nargs=1,
        default='OnBoot',
        help='Determines run behavior. Either "OnBoot" or "OnCommand"',
        required=True)
    
    # Other optional arguments
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
    elif args.run[0] == 'OnCommand':
        on_command(args)
    else:
        with open(ERRORSFILE, 'a+') as file:
            write_log(file, "Unknown run level specified")
        print "Unknown run level specified"


if __name__ == "__main__":
    main()