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

import logging
logging.basicConfig(filename = '/tmp/cistack.log',
    level = logging.INFO,
    format = '%(asctime)s %(levelname)s %(threadName)-10s %(message)s',)

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
import cistack as ci

def main():

    args = ci.readOpts()

    dt = datetime.datetime
    NOW = dt.isoformat(dt.now(), '-')
    logging.debug("Script started %s." % str(NOW))
    logging.info("Arguments have been parsed.")

    args = ci.readOpts()
    logging.debug("Command line arguments are: %s " % str(args))
    

# Target undeclared; no way to call cleanUp at this point.
    if not ci.startupChecks(args):
        logging.error("Startup checks failed.")
        sys.exit()

    logging.info("Creating a target object.")
    target = ci.getTarget(args.board)
    if not target:
        logging.error("Unable to determine board, or board is unsupported.")
        sys.exit()

    if args.ignoreGPIOwarnings:
        GPIO.setwarnings(False)

# the "get to work" part of the function
    if args.command == "flash":

# make a Binfile class object:
        logging.info("Creating a binfile object")
        b = ci.getBinfile(name = args.inputbinary, 
                path = args.binfilepath,
                board = args.board)
        if not b.validate():
            logging.error("Unable to validate binfile. Exiting.")
            ci.cleanUp(target, args)
        b.getInfo()

# set up the GPIO pins as requested
        target.setupboard()
        sleep(1)

# create a dict of Pin objects:
        pins = target.getpins()
        logging.debug(str(pins))

        target.powerup()
        sleep(0.5)

        target.reset()
        sleep(0.5)

        target.progmode()
        sleep(0.5)

        if (target.flash(b) is True):
            logging.info("Program flash completed. Reports success.")
        else:
            logging.error("Program flash may have failed.")
            ci.cleanUp(target, args)

        target.reset()
        logging.info("\nBoard reset.")

        sleep(1)

# If you want to shut the board down, this command cleans up and 
# de-energizes the power MOSFET, and frees the GPIO pins, if flagged
# to do so.
        ci.cleanUp(target, args)

    else:
        pass

if __name__ == '__main__':
   main()


#<EOF>
