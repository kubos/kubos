#!/usr/bin/python
#
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

pinvals = ci.pinLayout('pyboard-gcc')
p=pinvals

ci.setupBoard(**p)
print ("Board setup complete.")
sleep(1)

# should parse options here
# but instead:
binfile = "ukub-sensor-node.bin"
binpath = "/home/kubos"

ci.powerUp(**p)
print("Powering on the board")
sleep(1)

ci.setProg(**p)
print("Programming mode enabled")
sleep(1)

if (ci.flashBinary(binfile, binpath, **p) ):

    print("Program flash completed. Reports success.")

# declare a program return of 1 here so Jenkins can see success

else:
    print("Program flash appears to have failed.")

ci.resetBoard(**p)
print("\nBoard reset.")

# If you want to shut the board down, this command cleans up and 
# de-energizes the power MOSFET.
# ci.allDone()


#<EOF>
