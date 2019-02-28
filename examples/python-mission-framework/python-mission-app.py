#!/usr/bin/env python

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