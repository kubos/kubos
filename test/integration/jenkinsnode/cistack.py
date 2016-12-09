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
import os
import re
from time import sleep
import RPi.GPIO as GPIO
import subprocess
import smbus
import serial
import spidev


def allDone():
    '''Free all the pins and exit the script.'''
    GPIO.cleanup()
    sys.exit("Pins cleared. Exiting script.")
    return 1


def pinSetup(key, **pinvals):
    '''Set up one GPIO pin per the pin dict values. '''

    thepin = pinvals[key][0]

    if (thepin is False):
        return 1
    if (thepin == 'board'):
        return 1

    try:
        func = GPIO.gpio_function(thepin)
        print("pin %s set to %s" % (str(thepin), str(func) ) )
    except:
        sys.exit("Unable to determine the function of the pin!. Exiting.")        

    try:
        GPIO.setup(thepin, pinvals[key][3])
        print("Key %s, pin %s is set to %s " % 
            (str(key), str(thepin), str(pinvals[key][3] ) ) )
        sleep(0.5)
        return 1
    except:
        sys.exit("Unable to set the pin's function! Exiting.")

    return 0


def pinLayout(target):
    ''' Configure the board-specific pin addresses and directions '''
    if (target == "pyboard-gcc"):
        pinvals = {
        'board':target,
        'rst':[17, True, False, GPIO.OUT], 
        'prg':[18, True, False, GPIO.OUT],
        'pwr':[27, True, False, GPIO.OUT],
	    'opt':[22, True, False, GPIO.OUT]
        }
        return pinvals

    elif (target == "stm32f407-disco-gcc"):
        pinvals = {
        'board':target,
        'rst':[17, True, False, GPIO.OUT], # SWD connector, pull NRST to GND 
        'prg':[18, True, False, GPIO.OUT], # none needed?
        'pwr':[27, True, False, GPIO.OUT], # same as the rest of the hats
	    'opt':[22, True, False, GPIO.OUT]  # optional
        }
        return pinvals

    elif (target == "msp430f5529-gcc"):
        pinvals = {
        'board':target,
        'rst':[17, True, False, GPIO.OUT], # SWD connector, pull NRST to GND 
        'prg':[18, True, False, GPIO.OUT], # none needed?
        'pwr':[27, True, False, GPIO.OUT], # same as the rest of the hats
	    'opt':[22, True, False, GPIO.OUT]  # optional
        }
        return pinvals
    else:
        sys.exit("Unsupported board -- no pins available.")

    return 0


def resetBoard(**pinvals):
    '''Assert the reset pin for the board, then release it.'''
    pinOn('rst', **pinvals)
    sleep(0.5)
    pinOff('rst', **pinvals)
    sleep(0.5)
    return 1

def setProg(**pinvals):
    '''Determine which board is in use, and then set programming mode, if applicable, for that specific board. Boards not requiring a specific logical or physical assertion will do nothing but return success from the function.'''

    if (pinvals['board'] == "pyboard-gcc"):
        setProgPyBoard(**pinvals)

    elif (pinvals['board'] == "stm32f407-disco-gcc"):
    ### At present, we aren't using any pins to set programming mode;
    ### the embedded ST-Link circuit on the STM32F4 discovery board
    ### makes any pin asserts here unnecessary. However, the bootloader will
    ### eventually require pin asserts when we program over USART or other
    ### serial interfaces.   
        return 1

    elif (pinvals['board'] == "msp430f5529-gcc"):
    ### similarly, the MSP430 launchpad doesn't need any external pins because
    ### of the onboard programmer. However , in the future, TODO we will add
    ### support for JTAG or Spy-Bi-Wire programming
        return 1

    else:
        sys.exit("Unknown or unsupported board.")
        return 0
    return 1


# For the Micropython board specifically:
def setProgPyBoard (**pinvals):
    '''Assert two pins in sequence to enable programming mode on the 
MicroPython board, then release them to reboot the board into
programming mode.'''
    pinOn('rst', **pinvals)
    sleep(0.2)
    pinOn('prg', **pinvals)
    sleep(0.5)
    pinOff('rst', **pinvals)
    pinOff('prg', **pinvals)
    sleep(0.1)
    return 1


def powerUp(**pinvals):
    '''Turn on the power MOSFET for the target board.'''
    pinOn('pwr', **pinvals)
    sleep(0.1)
    return 1


def powerDown(**pinvals):
    '''Turn off the power MOSFET for the target board.'''
    pinOff('pwr', **pinvals)
    return 1


def pinOn(key, **pinvals):
    '''Generic "assert the GPIO pin" function.'''
    if (pinvals[key][0] is False):
        return 1
    try:
        print("Asserting pin %s" % str(pinvals[key][0])   )
        GPIO.output(pinvals[key][0], pinvals[key][1])
        
    except:
        print("ERROR: unable to assert pin %s" % str(pinvals[key][0])  )
        return 0
    return 1


def pinOff(key, **pinvals):
    '''Generic "turn off the GPIO pin" function.'''
    if (pinvals[key][0] is False):
        return 1
    try:
        GPIO.output(pinvals[key][0], pinvals[key][2])
    except:
        return 0
    return 1


# Setting BCM mode is "Broadcom", running from GPIO2 to GPIO27
# Meaning pin 40 in "BOARD" is pin 21 in BCM
def setupBoard(**pinvals):
    '''Run this function immediately after determining which pins
are assigned to your target board. This function sets the Raspberry
Pi GPIO map to "Broadcom" and then sets up pin direction and function.'''

    print("Checking for root access...")
    if os.geteuid() != 0:
        print("You need to have root privileges to run this script.\n")
        sys.exit("Please try again, this time using 'sudo'. Exiting.")
        return 0

    print("Setting pin modes for each pin:")
    GPIO.setmode(GPIO.BCM)
    for pin in pinvals.keys():
        if (pin == 'board'):
            pass
        else:
            pinSetup(pin, **pinvals)

    print("Setup successful.")
    sleep(0.1)
    return 1


# need to incorporate os.path.basename(__file__) NOTE  
def parseOptions(scriptname):
    if (sys.argv[0] == ""):
        print("ERROR: you need to provide a filename for the binary \
file and the name of the target board.")
        sys.exit("User did not provide required arguments.")

    parser = argparse.ArgumentParser(description = '''this python script 
uploads a compiled binary file for a specific target board. 

The script requires a minimum of two parameters, a filename for the binary 
file and the specific identifier for the target board / MCU.

Required parameters:

-f or --file       name of the binary to be flashed to the target board
-t or --target     the kubos-supported target board label (see below)

Optional parameters include:

-u or --url        the http-accessible location to download the binary file

Example:

python THIS_SCRIPT -f /path/to/your/file.bin -t pyboard-gcc  \n

Supported boards include:
pyboard-gcc
stm32f407-disco-gcc
msp430f5529-gcc
''')

# na-satbus-3c0-gcc
# kubos-msp430-gcc
# kubos-arm-none-eabi-gcc
# kubos-rt-gcc
# kubos-gcc
# stm32f405-gcc

    parser.add_argument("-f", "--file", \
        action = 'store', \
        dest = "binaryfilename", \
        default = "",
        help = "Provide a path and filename for Kubos binary executable.", \
            metavar = "FILE")

    parser.add_argument("-t", "--target", action = 'store', \
        dest = "target", \
        default = "pyboard-gcc", \
        help = "Identify the specific target board and microcontroller, \
as specified in the Kubos documentation.", metavar = "TARGET")

    parser.add_argument("-u", "--url", \
        action = 'store', \
        dest = "url", \
        default = "",
        help = "Provide an http download link for the binary.", \
            metavar = "URL")

    arguments = parser.parse_args()
    args = vars(arguments)

    return args


def flashBinary(binfile,binpath, \
flashpath = "/usr/local/lib/python2.7/dist-packages/kubos/flash/openocd",  \
**pinvals):
    '''This function determines which board is in use, and flashes the \
binary using an appropriate binary executable and/or shell script(s).'''
    if (pinvals['board'] == "pyboard-gcc"):
        if (flashBinaryPyBoard(binfile,binpath) == 1 ):
            return 1
        else:
            return 0
    elif(pinvals['board'] == "stm32f407-disco-gcc"):
        if (flashBinarySTM32F4DiscoveryBoard(binfile, binpath, flashpath) == 1):
            return 1
        else:
            return 0 

    elif(pinvals['board'] == "msp430f5529-gcc"):
        if (flashBinaryMSP430F5529(binfile, binpath, flashpath) == 1):
            return 1
        else:
            return 0 

    else:
        sys.exit("Unknown or unsupported board.")
        return 0
    return 0


# dfu-util --alt 0 -D /home/username/ukub-sensor-node.bin -i 0 -s 0x08000000
def flashBinaryPyBoard(binfile, binpath):
    '''use an external shell to push the binary file using dfu-util.'''
    bpath = "/home/youruserid"
    binpath = re.sub("/$", "", binpath)

    if (__pyBoardBinarySanityCheck(binfile)):
        pass
    else:
        sys.exit("Binary file didn't pass a sanity check. Exiting.")
        return 0

    dfupath = subprocess.check_output('/usr/bin/which dfu-util', shell = True)
    dfupath = re.sub('\n$', '', dfupath)

    tail = str("-i 0 -s 0x08000000")
    head = str("--alt 0 -D ")
    command = str("%s %s %s/%s %s " % 
            (dfupath, head, binpath, binfile, tail) )
    print(command)
    try:
        output = subprocess.check_output(command , shell = True )
        print(output)

        if (re.search("File downloaded successfully.*$", output ) ):
            print("Looks like things went well!")

    except:
        print "well, crap. Try it again."
        return 0
    sleep(0.5)
    return 1


def flashBinarySTM32F4DiscoveryBoard(binfile, binpath, flashpath):
    '''use an external shell to push the ELF file using openocd.'''
    
    # TODO: get this value dynamically:
    SEARCH   = "/usr/local/lib/python2.7/dist-packages/kubos/flash/openocd"
    PYPATH   = "/usr/local/lib/python2.7/dist-packages/kubos/flash/openocd"    
    STMFLASH = str("stm32f4_flash %s/%s" % (binpath, binfile) )
    SCRIPT   = "flash.sh"
    openocd  = subprocess.check_output('/usr/bin/which openocd', shell = True)
    cmdpath  = re.sub('\n$', '', openocd)
    
    command  = str("%s/%s %s %s %s" % 
        ( SEARCH, SCRIPT, cmdpath, STMFLASH, SEARCH ) )    
    
    print("\n** About to execute: \n\n%s\n" % command)

    try:
        print("\n** Flashing binary to the board:\n")
        output = subprocess.check_output(command, shell = True)
        print(str("\n\n========\n%s\n" %  output) )

    except:
        print("** ERROR: An unknown error occurred.")
        return 0

    try:
        print ("** Checking for successful flash.")

        if (re.search("\n0\n", output)):
            print("\n** Flash reports success.\n")

        else:
            print("\n** Cannot determine output success or failure.\n")
            return 0

    except:
        print "\n** Something went wrong with the flash. Try again.\n"
        return 0

    sleep(0.5)
    return 1

def flashBinaryMSP430F5529(binfile, binpath, flashpath):
    '''Flash a binary file through the USB connection on an MSP430 Launchpad'''
    PYPATH  = "/usr/local/lib/python2.7/dist-packages/kubos/flash/mspdebug"
    SCRIPT  = "flash.sh"
    DEBUGCOMMAND = "prog"

# base command:
# /path/to/mspdebug tilib "prog /path/to/binary" --allow-fw-update
#
# or, for the flash.sh:
#
# ./mspflash.sh /usr/local/lib/python2.7/dist-packages/kubos/bin/linux/mspdebug prog /home/kubos/kubos-rt-example 

    mspdebugloc = subprocess.check_output('/usr/bin/which mspdebug', 
        shell = True )
    cmdpath  = re.sub('\n$', '', mspdebugloc)
    command = str ("%s/%s %s %s %s/%s" % 
        ( PYPATH, SCRIPT, cmdpath, DEBUGCOMMAND, binpath, binfile ) )

    try:
        print("\n** Flashing binary to the board:\n")
        output = subprocess.check_output(command, shell = True)
        print(str("\n\n========\n%s\n" %  output) )

    except:
        print("** ERROR: An unknown error occurred.")
        return 0

    try:
        print ("** Checking for successful flash.")

        if (re.search("\n0\n", output)):
            print("\n** Flash reports success.\n")

        else:
            print("\n** Cannot determine output success or failure.\n")
            return 0

    except:
        print "\n** Something went wrong with the flash. Try again.\n"
        return 0

    sleep(0.5)
    return 1


def __pyBoardBinarySanityCheck(binfile):
    '''Ensure that the file ends in .bin, and not .elf. '''
    if (re.search("\.bin$", binfile)):
        return 1
    else:
        return 0


def __discoBoardBinarySanityCheck(binfile):
    '''Ensure that -- for now -- the binary file to be flashed is an .elf, \
not a .bin file. It seems that .elf files know where to go, because of the \
debugging information; bin files lack that information. One problem is that \
.elf files usually don't have file name suffixes, meaning it cannot be regexed.'''
    if (re.search("\.bin$", binfile)):
        return 0
    else:
        return 1

    

#####################################################################
# NOTE: these are placeholders at present.
#####################################################################


def sanityChecks(): # How do we check for executables in Windows?
    '''Check for dfu-util and other stuff.'''

    return 0


def scpBinary(filename, url, certificate, savepath, userid):
    '''Go and obtain the binary for upload using SCP.'''

    return 0


def curlBinary(filename, url, savepath):
    '''Go and obtain the binary for upload from a given url.'''

    return 0 


def uploadSerial(filename):
    '''Send a binary file over the RPi UART.'''

    return 0


#<EOF>
