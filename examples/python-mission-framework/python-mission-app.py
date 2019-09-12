#!/usr/bin/env python3

import app_api
import argparse
import sys

def on_boot(logger):
    
    logger.info("OnBoot logic")
    
def on_command(logger):
    
    logger.info("OnCommand logic")

def main():

    logger = app_api.logging_setup("mission-framework")
    
    parser = argparse.ArgumentParser()
    
    parser.add_argument('--run', '-r', nargs=1)
    parser.add_argument('--config', '-c', nargs=1)
    parser.add_argument('cmd_args', nargs='*')
    
    args = parser.parse_args()
    
    if args.config is not None:
        global SERVICES
        SERVICES = app_api.Services(args.config[0])
    else:
        SERVICES = app_api.Services()
    
    if args.run[0] == 'OnBoot':
        on_boot(logger)
    elif args.run[0] == 'OnCommand':
        on_command(logger)
    else:
        logger.error("Unknown run level specified")
        sys.exit(1)
    
if __name__ == "__main__":
    main()