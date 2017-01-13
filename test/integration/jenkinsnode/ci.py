#!/usr/bin/python

# Kubos Continuous Integration
# Copyright (C) 2016 Kubos Corporation
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#
# cistack.py: a Raspberry Pi interface library specific to 
# KubOS-supported OBC and SBC products.

import sys
# sys.path.append('/var/lib/ansible/')
import os
import re
from time import sleep
import RPi.GPIO as GPIO
import subprocess
import smbus2
import serial
import spidev
import argparse
import datetime
import magic 
import logging
import cistack as ci

global shutdown
global freepins

def main():

    log = logging.getLogger('logfoo')
    dt = datetime.datetime
    NOW = dt.isoformat(dt.now(), '-')
    log.debug("Script started %s." % str(NOW))

    args = ci.readOpts()
    log.debug("Command line arguments are: %s " % str(args))


    log.debug("Checking for required system utilities.")
    requiredutils = ci.requiredUtils()

    if ci.sanityChecks(*requiredutils):
        log.info("Located required system utilities. Continuing.")
    else:
        log.exception("Unable to locate required utilites. Exiting.")

    paths = ci.requiredPaths()
    if ci.pathChecks(*paths):
        log.info("Located required system environment variables. Continuing.")
    else:
        log.error("Unable to locate required environment variables. Exiting.")
        sys.exit() # haven't even created a target yet, so this is the way out.

# instantiate some Target class object:
    target = ci.getTarget(args.board)

    if args.command == "lib":
        pass

    if args.ignoreGPIOwarnings:
        GPIO.setwarnings(False)


    if args.command == "flash":

# instantiate some Binfile class object:
        log.info("Creating a binfile object")
        b = ci.getBinfile(name = args.inputbinary, 
                path = args.binfilepath,
                board = args.board)
        b.validate()
        b.getInfo()

        target.setupboard()
        log.info("Board setup complete.")
        sleep(1)

        pins = target.getpins()
        print (str(pins))

        target.powerup()
        log.debug("Powering on the board")
        sleep(1)

        target.reset()
        sleep(1)

        target.progmode()
        sleep(1)

        if (target.flash(b) is True):
            log.info("Program flash completed. Reports success.")
        else:
            log.error("Program flash may have failed.")
            cleanUp(target)

        target.reset()
        log.info("\nBoard reset.")

        sleep(1)

        if(args.shutdown):
            log.info("Shutting down the board.")
            target.powerdown() 

# If you want to shut the board down, this command cleans up and 
# de-energizes the power MOSFET.
        if(args.freepins):
            log.info("Freeing pins and exiting.") 
            allDone()

        logging.shutdown()

    else:
        pass



if __name__ == '__main__':
   main()


#<EOF>
