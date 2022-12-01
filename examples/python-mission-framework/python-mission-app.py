#!/usr/bin/env python3

from kubos import app
import argparse
import sys

def main():

    logger = app.logging_setup("mission-framework")
    
    parser = argparse.ArgumentParser()
    
    parser.add_argument('--config', '-c', nargs=1)
    
    args = parser.parse_args()
    
    if args.config is not None:
        global SERVICES
        SERVICES = app.Services(args.config[0])
    else:
        SERVICES = app.Services()
    
    logger.info("Starting mission logic")
    
if __name__ == "__main__":
    main()
