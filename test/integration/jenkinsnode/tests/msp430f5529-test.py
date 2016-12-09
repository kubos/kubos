#!/usr/bin/python
import sys
import os
from time import sleep
import RPi.GPIO as GPIO
import cistack as ci
import subprocess

pinvals = ci.pinLayout('msp430f5529-gcc')
p=pinvals

ci.setupBoard(**p)
print ("Board setup complete.")
sleep(1)

# should parse options here
# but instead:
binfile = "kubos-rt-example"
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
