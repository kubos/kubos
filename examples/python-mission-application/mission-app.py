#!/usr/bin/env python3

# Copyright 2018 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.

"""
Example Python mission application showing how to interact with onboard services.

This application is meant to be run through the mission application
service, but can be run directly from the command line as well.

NOTE: Mission application service will NOT run Python mission apps
without the environment indicator at the top of the file:
"#!/usr/bin/env python3"
"""

import argparse
from kubos import app
import sys
import time
import toml

# Enter system safemode
def safe_mode(logger, time):
    if time > 0:
        logger.info(
            "Going into safemode for {} seconds".format(
                args.time))
        logger.info("Sending commands to hardware to go into safemode")
        time.sleep(args.time)
        logger.info("Sending commands to hardware to normal operation")
    else:
        raise ValueError("Command Integer must be positive and non-zero")
        sys.exit(1)
        
# Query the apps service for all installed apps
def get_apps(logger):
    
    query = '{ apps { active, app { name, version, author } } }'
    try:
        logger.info("Querying for active applications")
        logger.info("Query: {}".format(query))
        apps = SERVICES.query(service="app-service", query=query)
        logger.info("Active applications are: {}".format(apps))
    except Exception as e:
        logger.error("Housekeeping caused an error: {},{},{}".format(
            type(e), e.args, e))

# Gather system telemetry and store it in the telemetry database
def get_telemetry(logger):
        
    logger.info("Gathering telemetry")

    # Get the current amount of available memory from the monitor service
    try:
        request = '{memInfo{available}}'
        response = SERVICES.query(service="monitor-service", query=request)
    except Exception as e: 
        logger.error("Something went wrong: " + str(e) + "")
        return
        
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
        return
        
    data = response["insert"]
    success = data["success"]
    errors = data["errors"]
    
    if success == False:
        logger.error("Telemetry insert encountered errors: " + str(errors) + "")
    else:
        logger.info("Telemetry insert completed successfully")
        
def main():
   
    logger = app.logging_setup("mission-app")
    
    parser = argparse.ArgumentParser()

    # The -c argument should be present if you would like to be able to specify a non-default
    # configuration file
    parser.add_argument(
        '-c',
        '--config',
        nargs=1,
        help='Specifies the location of a non-default configuration file')
    # Arguments specific to this application
    parser.add_argument(
        '-a',
        '--apps',
        action='store_true',
        help='Get list of installed apps')
    parser.add_argument(
        '-m',
        '--mode',
        nargs=1,
        help='System mode')
    parser.add_argument(
        '-t',
        '--time',
        type=int,
        help='Safemode time (in seconds)')

    args = parser.parse_args()
    
    if args.config is not None:
        global SERVICES
        SERVICES = app.Services(args.config[0])
    else:
        SERVICES = app.Services()

    if args.mode == 'safemode':
        safe_mode(logger, args.time)
    elif args.apps is not None:
        get_apps(logger)
    else:
        get_telemetry(logger)

if __name__ == "__main__":
    main()
