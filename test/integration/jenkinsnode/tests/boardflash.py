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
# Depending on your udev rules (see /etc/udev/rules.d/ ) you may have to
# run this script as root
#

# Should execute this script with three command line arguments:
# -f compiled_binary_file_to_upload
# -p /path/to/compiled/binary/file
# -b typeofboard ( that is, stm32f407-disco-gcc, pyboard-gcc, or msp430f5529-gcc ) 
#


import sys
import os
from time import sleep
import RPi.GPIO as GPIO
import cistack as ci
import subprocess

kwargs = ci.readOpts()

print(str(kwargs))

# 'inputbinary': 'kubos-rt-example', 'binfilepath': '/home/kubos', 'verbose': True, 'board': 'stm32f407-disco-gcc'
binfile = kwargs['inputbinary']
binpath = kwargs['binfilepath']
board = kwargs['board']

pinvals = ci.pinLayout(board)
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

# New flash subroutine:



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
