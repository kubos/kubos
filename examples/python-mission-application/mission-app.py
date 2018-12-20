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
import sys
import time
import toml

SERVICES = app_api.Services()

# On-boot logic which will be called at boottime if this app is registered with
# the applications service
def on_boot(logger):
        
    logger.info("OnBoot logic")

    while True:
        # Get the current amount of available memory from the monitor service
        try:
            request = '{memInfo{available}}'
            response = SERVICES.query(service="monitor-service", query=request)
        except Exception as e: 
            logger.error("Something went wrong: " + str(e) + "")
            continue
        
        data = response["memInfo"]
        available = data["available"]
        
        logger.info("Current available memory: %s kB" % (available))
        
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
            logger.error("Something went wrong: " + str(e) + "")
            continue
            
        data = response["insert"]
        success = data["success"]
        errors = data["errors"]
        
        if success == False:
            logger.error("Telemetry insert encountered errors: " + str(errors) + "")
        else:
            logger.info("Telemetry insert completed successfully")
        
        # Wait five minutes before checking again
        time.sleep(300)

# On-demand logic which will be called manually by the user (potentially via the applications service)
def on_command(logger, cmd_args):

    logger.info("OnCommand logic")

    if cmd_args.cmd_string == 'safemode':
        if cmd_args.cmd_int > 0:
            with open(LOGFILE, 'a+') as file:
                logger.info(
                    "Going into safemode for {} seconds".format(
                        cmd_args.cmd_int))
                logger.info("Sending commands to hardware to go into safemode")
                time.sleep(cmd_args.cmd_int)
                logger.info("Sending commands to hardware to normal operation")
        else:
            raise ValueError("Command Integer must be positive and non-zero")
            sys.exit(1)
                    
    else:
        query = '{ apps { active, app { uuid, name, version, author } } }'
        try:
            logger.info("Querying for active applications")
            logger.info("Query: {}".format(query))
            apps = SERVICES.query(service="app-service", query=query)
            logger.info("Active applications are: {}".format(apps))
        except Exception as e:
            logger.error("Housekeeping caused an error: {},{},{}".format(
                type(e), e.args, e))
            sys.exit(1)
        

def main():
   
    logger = app_api.logging_setup("mission-app")
    
    parser = argparse.ArgumentParser()

    # The -r argument is required to be present by the applications service
    parser.add_argument(
        '-r',
        '--run',
        nargs=1,
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
        on_boot(logger)
    elif args.run[0] == 'OnCommand':
        on_command(logger, args)
    else:
        logger.error("Unknown run level specified")
        sys.exit(1)


if __name__ == "__main__":
    main()