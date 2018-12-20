#!/usr/bin/env python

import argparse
import logging
from logging.handlers import SysLogHandler
import sys

def on_boot(logger):
    
    logger.info("OnBoot logic")
    
def on_command(logger):
    
    logger.info("OnCommand logic")

def main():

    # Create a new logger
    logger = logging.getLogger('mission-framework')
    # We'll log everything of Debug level or higher
    logger.setLevel(logging.DEBUG)
    # Set the log message template
    formatter = logging.Formatter('mission-framework: %(message)s')
    
    # Set up a handler for logging to syslog
    syslog = SysLogHandler(address='/dev/log', facility=SysLogHandler.LOG_USER)
    syslog.setFormatter(formatter)
    
    # Set up a handler for logging to stdout
    stdout = logging.StreamHandler(stream=sys.stdout)
    stdout.setFormatter(formatter)
    
    # Finally, add our handlers to our logger
    logger.addHandler(syslog)
    logger.addHandler(stdout)
    
    parser = argparse.ArgumentParser()
    
    parser.add_argument('--run', '-r')
    
    args = parser.parse_args()
    
    if args.run == 'OnBoot':
        on_boot(logger)
    elif args.run == 'OnCommand':
        on_command(logger)
    else:
        logger.error("Unknown run level specified")
        sys.exit(1)
    
if __name__ == "__main__":
    main()