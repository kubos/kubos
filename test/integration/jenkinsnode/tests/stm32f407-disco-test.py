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
#
# Must run as root!
#

import sys
import os
from time import sleep
import RPi.GPIO as GPIO
import cistack as ci
import subprocess

pinvals = ci.pinLayout('stm32f407-disco-gcc')
p = pinvals

flashloc = "/usr/local/lib/python2.7/dist-packages/kubos/flash/"
freepinswhendone = 0
shutdownwhendone = 0
ignoreGPIOwarnings = False

if (ignoreGPIOwarnings):
    GPIO.setwarnings(False)


ci.setupBoard(**p)
print ("Board setup complete.")
sleep(1)

# should parse options here
# but instead:
binfile = "disco-ukub-sensor-node"
binpath = "/home/kubos"

ci.powerUp(**p)
print("Powering on the board")
sleep(1)

ci.setProg(**p)
print("Programming mode not needed for this board at this moment.")
sleep(1)

if (ci.flashBinary(binfile, binpath, flashpath = flashloc,  **p) ):

    print("Program flash completed. Reports success.")

# program returns 0 here so Jenkins can see success

else:
    print("Program flash appears to have failed.")

ci.resetBoard(**p)
print("\nBoard reset.")

sleep(1)

if(shutdownwhendone):
    print("Shutting down the board.")
    ci.powerDown(**p)

# If you want to shut the board down, this command cleans up and 
# de-energizes the power MOSFET.
if(freepinswhendone):
    print("Freeing pins and exiting.") 
    ci.allDone()



#<EOF>
