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
import cistack as ci

def main():

    kwargs = ci.readOpts()
    print(str(kwargs))

    global binfile, binpath, board, isrootrequired
    binfile = kwargs['inputbinary']
    binpath = kwargs['binfilepath']
    board = kwargs['board']
    isrootrequired = kwargs['root']
    shutdownwhendone = kwargs['shutdown']
    freepinswhendone = kwargs['freepins']
    ignoreGPIOwarnings = kwargs['ignoreGPIOwarnings']
    command = kwargs['command']
    
    requiredutils = ci.requiredUtils()

    if ci.sanityChecks(*requiredutils):
        print("Located required system utilities. Continuing.")
    else:
        sys.exit("Unable to locate required utilites. Exiting.")

    pathkeys = ci.requiredPaths()
    pathdict = dict.fromkeys(pathkeys, "")
    if ci.pathChecks(**pathdict):
        print("Located required system environment variables. Continuing.")
    else:
        sys.exit("Unable to locate required environment variables. Exiting.")

# instantiate some Target class object:
    target = ci.getTarget(board)


# instantiate some Binfile class object:
    b = ci.getBinfile(name = binfile, path = binpath, board = board)
    b.validate()
    b.getInfo()

    if (ignoreGPIOwarnings):
        GPIO.setwarnings(False)


    if command == "flash":

        target.setupboard()
        print ("Board setup complete.")
        sleep(1)

        pins = target.getpins()
        print (str(pins))

        target.powerup()
        print("Powering on the board")
        sleep(1)

        target.reset()
        sleep(1)

        target.progmode()
        sleep(1)

        if (target.flash(b) is True):
            print("Program flash completed. Reports success.")
        else:
            sys.exit("Program may have failed. Exiting.")

        target.reset()
        print("\nBoard reset.")

        sleep(1)

        if(shutdownwhendone):
            print("Shutting down the board.")
            target.powerdown() 

# If you want to shut the board down, this command cleans up and 
# de-energizes the power MOSFET.
        if(freepinswhendone):
            print("Freeing pins and exiting.") 
            allDone()

    else:
        pass



if __name__ == '__main__':
   main()


#<EOF>
