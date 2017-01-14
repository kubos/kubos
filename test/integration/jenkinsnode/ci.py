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

    args = ci.readOpts(
    log.debug("Command line arguments are: %s " % str(args))

    if not ci.startupChecks(args):
        log.error("Startup checks failed.")
        sys.exit()

    log.info("Creating a target object.")
    target = ci.getTarget(args.board)
    if not target:
        log.error("Unable to determine board, or board is unsupported.")
        sys.exit()

    if args.ignoreGPIOwarnings:
        GPIO.setwarnings(False)

# the "get to work" part of the function
    if args.command == "flash":

# make a Binfile class object:
        log.info("Creating a binfile object")
        b = ci.getBinfile(name = args.inputbinary, 
                path = args.binfilepath,
                board = args.board)
        if not b.validate():
            log.error("Unable to validate binfile. Exiting.")
            ci.cleanUp()
        b.getInfo()


        target.setupboard()
        sleep(1)

        pins = target.getpins()
        log.debug(str(pins))

        target.powerup()
        sleep(0.5)

        target.reset()
        sleep(0.5)

        target.progmode()
        sleep(0.5)

        if (target.flash(b) is True):
            log.info("Program flash completed. Reports success.")
        else:
            log.error("Program flash may have failed.")
            ci.cleanUp(target)

        target.reset()
        log.info("\nBoard reset.")

        sleep(1)

# If you want to shut the board down, this command cleans up and 
# de-energizes the power MOSFET, and frees the GPIO pins, if flagged
# to do so.
        ci.cleanUp(target)

    elif args.command == "lib":
        pass

#TODO add more behaviors
    else:
        pass

if __name__ == '__main__':
   main()


#<EOF>
