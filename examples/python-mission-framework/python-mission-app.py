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

    logger = logging.getLogger('mission-framework')
    logger.setLevel(logging.INFO)
    
    handler = SysLogHandler(address='/dev/log', facility=SysLogHandler.LOG_USER)
    
    formatter = logging.Formatter('mission-framework: %(message)s')
    
    handler.formatter = formatter
    logger.addHandler(handler)
    
    parser = argparse.ArgumentParser()
    
    parser.add_argument('--run', '-r')
    
    args = parser.parse_args()
    
    if args.run == 'OnBoot':
        on_boot(logger)
    elif args.run == 'OnCommand':
        on_command(logger)
    else:
        logger.error("Unknown run level specified")
        print "Unknown run level specified"
        sys.exit(1)
    
if __name__ == "__main__":
    main()