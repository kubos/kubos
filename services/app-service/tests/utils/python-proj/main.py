#!/usr/bin/env python3

# Copyright 2019 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.

import app_api
import argparse
import logging
from sub import sub
import sys


def on_boot():
    
    sub.test_func()
    sys.exit(0)
    
def on_command(args):
    
    parser = argparse.ArgumentParser()
    
    parser.add_argument('-t', '--test', nargs=1)
    parser.add_argument('-f', '--flag', action='store_true')
    parser.add_argument('positional', nargs='?')
    
    matches = parser.parse_args(args)
    
    success = False
    
    if matches.flag:
        success = True
        
    if matches.test is not None and matches.test[0] == "test":
        success = True
        
    if matches.positional is not None and matches.positional == "pos":
        success = True
        
    if success:
        sys.exit(0)
    else:
        logging.error("No valid arguments were found")
        sys.exit(1)

def main():

    parser = argparse.ArgumentParser()
    
    parser.add_argument(
        '-r',
        '--run',
        nargs=1,
        help='Determines run behavior. Either "OnBoot" or "OnCommand"',
        required=True)
    parser.add_argument(
        '-c',
        '--config',
        nargs=1,
        help='Specifies the location of a non-default configuration file')
    parser.add_argument('cmd_args', nargs='*')
    
    args = parser.parse_args()
    
    if args.config is not None:
        global SERVICES
        SERVICES = app_api.Services(args.config[0])
    else:
        SERVICES = app_api.Services()

    if args.run[0] == 'OnBoot':
        on_boot()
    elif args.run[0] == 'OnCommand':
        on_command(args.cmd_args)
    else:
        print("Unknown run level specified")
        sys.exit(1)
    
if __name__ == "__main__":
    main()
