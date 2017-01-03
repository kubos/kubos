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

errstr="*** ERROR:"

help=str("cistack.py; a python library that provides a series of abstracted\
functions to interact with supported KubOS target boards, through the \
Kubos Pi Hat v0.3 interface. The library provides numerous functions,\
but chief among them is the ability to upload a compiled\
binary executable to each board using the flashing functions in the library.\
As such, the user must provide, at a minimum, three arguments to any\
script that calls this (readOpts) function.") 

supportedboards=[
'pyboard-gcc',
'stm32f407-disco-gcc',
'msp430f5529-gcc'
]


def allDone():
    '''Free all the pins and exit the script.'''
    GPIO.cleanup()
    sys.exit("Pins cleared. Exiting script.")
    return True



class Target(object):

    def __init__(self):
        self.board = ""
        self.pins = {}

    def getboard(self):
        return self.board

    def getpins(self):
        return self.pins

    def progmode(self):
        return False

    def flash(self, binfile, binpath):
        return False

    def reset(self):
        '''Assert the reset pin for the board, then release it.'''
        pinOn('rst', **self.pins)
        sleep(0.5)
        pinOff('rst', **self.pins)
        sleep(0.5)
        return True

    def powerup(self):
        '''Turn on the power MOSFET for the target board.'''
        pinOn('pwr', **self.pins)
        sleep(0.1)
        return True

    def powerdown(self):
        '''Turn off the power MOSFET for the target board.'''
        pinOff('pwr', **self.pins)
        return True

    def setupboard(self):
        '''Run this function immediately after determining which pins
are assigned to your target board. This function sets the Raspberry
Pi GPIO map to "Broadcom" and then sets up pin direction and function.'''

#    print("Checking for root access...")
#    if os.geteuid() != 0:
#        print("You need to have root privileges to run this script.\n")
#        sys.exit("Please try again, this time using 'sudo'. Exiting.")
#        return False

# Setting BCM mode is "Broadcom", running from GPIO2 to GPIO27
# Meaning pin 40 in "BOARD" is pin 21 in BCM
        print("Setting pin modes for each pin:")
        GPIO.setmode(GPIO.BCM)

        for pin in self.pins.keys():
            pinSetup(pin, **self.pins)

        print("Setup successful.")
        sleep(0.1)
        return True



class Pyboard(Target):
    def __init__(self):
        self.board = "pyboard-gcc"
        self.pins = {
            'rst':[17, True, False, GPIO.OUT], 
            'prg':[18, True, False, GPIO.OUT],
            'pwr':[27, True, False, GPIO.OUT],
            'opt':[22, True, False, GPIO.OUT]
        }

    def progmode(self):
        setupPyBoard(self.pins)

    def flash(self, binfile, binpath):
        '''use an external shell to push the binary file using dfu-util.'''

        if (__pyBoardBinarySanityCheck(binfile)):
            pass
        else:
            sys.exit("Binary file didn't pass a sanity check. Exiting.")
            return False

        dfupath = findBin('dfu-util')

# Note that the pyboard must be in programming mode for it to announce itself
# as a DFU device. That is, the system can't even find the board until it is
# in programming mode.
# This script no longer requires root access to run, meaning you must implement
# a udev rule to enable user-land access to the device, or else just remember
# to sudo when running this script.
# 
# Hint: ATTRS{idVendor}==\"0483\", ATTRS{idProduct}==\"df11\"

        tail = str("-i 0 -s 0x08000000")
        head = str("--alt 0 -D ")
        command = str("%s %s %s/%s %s " % 
            (dfupath, head, binpath, binfile, tail))
        print(command)
        try:
            output = subprocess.check_output(command , shell = True)
            print(output)

            if (re.search("File downloaded successfully.*$", output)):
                print("Looks like things went well!")

        except:
            print "NOPE. Try it again."
            return False

        sleep(0.5)
        return True




class MSP430(Target):
    def __init__(self):
        self.board = "msp430f5529-gcc"
        self.pins = {
            'rst':[17, True, False, GPIO.OUT], # SWD connector, pull NRST to GND 
            'prg':[18, True, False, GPIO.OUT], # none needed?
            'pwr':[27, True, False, GPIO.OUT], # same as the rest of the hats
            'opt':[22, True, False, GPIO.OUT]  # optional
        }
    
    def flash(self,binfile,binpath):
        '''Flash a binary file through the USB connection on an MSP430 Launchpad'''
        searchpath = "/usr/local/lib/python2.7/dist-packages/kubos/lib/linux/"
        sp1 = os.environ['LD_LIBRARY_PATH']
        sp1 = str(sp1 + ":" + searchpath)

        fileloc=str("%s/%s" % (binpath, binfile))
        print("LD_LIBRARY_PATH will be: %s" % str(sp1))
        print("File to be flashed: %s" % str(fileloc))

        mspdebugloc = findBin('mspdebug') 
        print("Found mspdebug at %s" % mspdebugloc)
        cmd = "prog" 
    
        commandstring = str("LD_LIBRARY_PATH=\"%s\"   %s tilib \"%s %s\" --allow-fw-update" % 
            (sp1, mspdebugloc, cmd, fileloc))

        print "About to execute the following command:\n%s" % str(commandstring)

        try:
            print("\n** Flashing binary to the board:\n")
            output = subprocess.check_output(commandstring, shell = True)
            print(str("\n\n========\n%s\n" %  output))

        except:
            print ("%s An unknown error occurred." % errstr)
            return False

        return True 


class STM32F407Discovery(Target):
    def __init__(self):
        self.board = "stm32f407-disco-gcc"
        self.pins = {
            'rst':[17, True, False, GPIO.OUT], # SWD connector, pull NRST to GND 
            'prg':[18, True, False, GPIO.OUT], # none needed?
            'pwr':[27, True, False, GPIO.OUT], # same as the rest of the hats
            'opt':[22, True, False, GPIO.OUT]  # optional
        }

# IMPORTANT NOTE: openocd must be version 0.9 or later.
    def flash(self, binfile, binpath):
        '''use an external shell to push the ELF file using openocd. It seems 
        to be necessary to pre-declare the LIB PATH for some commands, and 
        if the path variable is not available as declared in /etc/profile, it
        can be fixed here with the sp1 variable, below. HOWEVER: from ansible,
        the locally-declared and locally-requested path variables DO NOT WORK
        and cause ERRORS. Workaround: use the ansible -shell- command and 
        declare the library path before executing a bash -c command'''

        distpath="/usr/local/lib/python2.7/dist-packages/kubos"
        searchpath=str("%s/flash/openocd" % distpath)
   
        KUBOS_LIB_PATH=str("%s/lib/linux/" % distpath)
        sp1 = os.environ['LD_LIBRARY_PATH']
        sp1 = str(sp1 + ":" + KUBOS_LIB_PATH)
        sp1 = str(sp1 + ":" + searchpath)

        openocdloc = findBin('openocd')

        unamestr = subprocess.check_output('uname')
        unamestr = re.sub('\n$', '', unamestr)

# TODO adjust the paths for OS X

#    if (re.search('Linux', unamestr)):
# /usr/bin/openocd  -f /usr/local/lib/python2.7/dist-packages/kubos/flash/openocd/stm32f407vg.cfg   -s /usr/local/lib/python2.7/dist-packages/kubos/flash/openocd -c "stm32f4_flash /home/kubos/kubos-rt-example"

###    cfg = "stm32f407g-disc1.cfg"
#    cfg = "stm32f407vg.cfg"
#    cmd = "stm32f4_flash"

# At present, this function only expects one board to be attached. TODO
        boards = whichUSBboard()
        configs = getBoardConfigs(boards)
        cfg = configs[2] # config file to use with openocd
        cmd = configs[3] # something like 'stm32f4_flash', an openocd command

# $openocd -f $this_dir/$cfg -s $search_path -c "$cmd $file"
        command = str("%s -f %s/%s -s %s -c \"%s %s/%s\"") % (openocdloc, 
            searchpath, cfg, searchpath, cmd, binpath, binfile)
        print (str(command))
        try:
            subprocess.check_output(command, shell=True)
            return True
        except:
            return False

def pinSetup(key, **pinvals):
    '''Set up one GPIO pin per the pin dict values. '''
    thepin = pinvals[key][0]
    if (thepin is False):
        return True
    if (thepin == 'board'):
        return True

    try:
        func = GPIO.gpio_function(thepin)
        print("pin %s set to %s" % (str(thepin), str(func)))
    except:
        sys.exit("Unable to determine the function of the pin!. Exiting.") 

    try:
        GPIO.setup(thepin, pinvals[key][3])
        print("Key %s, pin %s is set to %s " % 
            (str(key), str(thepin), str(pinvals[key][3])))
        sleep(0.5)
        return True
    except:
        sys.exit("Unable to set the pin's function! Exiting.")

    return False

def getTarget(target):
    ''' Configure the board-specific pin addresses and directions '''
    if (target == "pyboard-gcc"):
        return Pyboard()

    elif (target == "stm32f407-disco-gcc"):
        return STM32F407Discovery() 

    elif (target == "msp430f5529-gcc"):
        return MSP430()

    else:
        sys.exit("Unsupported board -- no pins available.")

    return None



def setProg(**pinvals):
    '''Determine which board is in use, and then set programming mode, 
if applicable, for that specific board. Boards not requiring a specific 
logical or physical assertion will do nothing but return success from 
the function.'''

    if (pinvals['board'] == "pyboard-gcc"):
        setProgPyBoard(**pinvals)

    elif (pinvals['board'] == "stm32f407-disco-gcc"):
    ### At present, we aren't using any pins to set programming mode;
    ### the embedded ST-Link circuit on the STM32F4 discovery board
    ### makes any pin asserts here unnecessary. However, the bootloader will
    ### eventually require pin asserts when we program over USART or other
    ### serial interfaces.   
        return True

    elif (pinvals['board'] == "msp430f5529-gcc"):
    ### similarly, the MSP430 launchpad doesn't need any external pins because
    ### of the onboard programmer. However , in the future, TODO we will add
    ### support for JTAG or Spy-Bi-Wire programming
        return True

    else:
        sys.exit("Unknown or unsupported board.")
        return False
    return True


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
    return True



def powerDown(**pinvals):
    '''Turn off the power MOSFET for the target board.'''
    pinOff('pwr', **pinvals)
    return True


def pinOn(key, **pinvals):
    '''Generic "assert the GPIO pin" function.'''
    if (pinvals[key][0] is False):
        return True
    try:
        print("Asserting pin %s" % str(pinvals[key][0]))
        GPIO.output(pinvals[key][0], pinvals[key][1])
        
    except:
        print("%s unable to assert pin %s" % str(errstr, pinvals[key][0]))
        return False
    return True


def pinOff(key, **pinvals):
    '''Generic "turn off the GPIO pin" function.'''
    if (pinvals[key][0] is False):
        return True
    try:
        GPIO.output(pinvals[key][0], pinvals[key][2])
    except:
        return False
    return True


def setupBoard(**pinvals):
    '''Run this function immediately after determining which pins
are assigned to your target board. This function sets the Raspberry
Pi GPIO map to "Broadcom" and then sets up pin direction and function.'''

#    print("Checking for root access...")
#    if os.geteuid() != 0:
#        print("You need to have root privileges to run this script.\n")
#        sys.exit("Please try again, this time using 'sudo'. Exiting.")
#        return False

# Setting BCM mode is "Broadcom", running from GPIO2 to GPIO27
# Meaning pin 40 in "BOARD" is pin 21 in BCM
    print("Setting pin modes for each pin:")
    GPIO.setmode(GPIO.BCM)

    for pin in pinvals.keys():
        if (pin == 'board'):
            pass
        else:
            pinSetup(pin, **pinvals)

    print("Setup successful.")
    sleep(0.1)
    return True


def determineFileType(binfile):
    '''Returns file type and useful info about the binary to be flashed.'''
    d=magic.from_file("/home/kubos/kubos-rt-example")
    d=re.sub(', ',',',d)
    e=d.split(',')
    return e

def __pyBoardBinarySanityCheck(binfile):
    '''Ensure that the file is not an .elf. '''
    e=determineFileType(binfile)
    t='data'
    if (re.search('data$', e[0])):
        return True
    else:
        return False


def __discoBoardBinarySanityCheck(binfile):
    '''Ensure that -- for now -- the binary file to be flashed is an .elf,
not a .bin file. It seems that .elf files know where to go, because of the 
debugging information; bin files lack that information. One problem is that 
.elf files usually don't have file name suffixes, meaning it cannot be 
simply found with a regex.'''
# ELF 32-bit LSB executable, ARM,
    e=determineFileType(binfile)
    try:
        if (re.search("^ELF",e[0]) and (e[1]=="ARM")):
            return True
    except:
        return False

    return False


def getBoardConfigs(boards):
    for i in boards:
        try:
            r=parseBoardIdentifier(i['dev'])
            if(r[1] is True): # board is supported
                return r
        except:
            sys.exit("Unable to determine board type. Exiting.")

    return False


def parseBoardIdentifier(lsusbpattern):
    '''Parse the lsusb identifier assigned to the board. Note that some boards
such as the PyBoard won't be enumerated by lsusb unless they are in 
DFU programming mode. Array values are as follows:
key: lsusb identifier
0: name of the device
1: Is the board supported by Kubos?
2: the configuration file for use by the flasher, if any
3: the command or argument specific to the board (mostly for openocd right now)
'''

# '0483:3748':['STMicro ST-LINK/V2 ',True, 'stm32f407vg.cfg', 'stm32f4_flash'],
# But note that the STLINK-V2 could be connected to many different boards. FIXME
    patterns={
              '0483:3748':['STMicro ST-LINK/V2 ',True, 'stm32f407vg.cfg', 'stm32f4_flash'],
              '0483:374b':['STMicro ST-LINK/V2 ',True, 'stm32f407g-disc1.cfg', 'stm32f4_flash'],
              '0483:df11':['STM32F405 PyBoard', True, 'USE dfu-util!', '***'], 
              '0451:2046':['TI MSP430F5529 Launchpad',True, 'USE mspdebug!', '***'],
              '0451:f432':['TI MSP430G2553 Launchpad',False, 'NOT SUPPORTED', '/usr/bin/sleep 1']
              }

    try:
        return patterns[lsusbpattern]
    except:
        return None

# kludgy at best, but helps. TODO replace with something better
def whichUSBboard():
    lsusb = findBin('lsusb')
    output = subprocess.check_output(lsusb, shell=True)
    lines = output.rsplit('\n')
    retarray = []
    manlist=['Texas', 'STMicro']

    for line in lines:
        arr = line.split(' ')
        for manuf in manlist:
            try:
                if (re.search(manuf, arr[6])):
                    print "found %s device at %s" % (manuf, arr[5])
                    retarray.append({ 'manuf':arr[6], 'dev':arr[5]})
            except:
                next

# TODO add more board identifiers here
    return retarray


def findBin(command):
    cmd = str("/usr/bin/which %s" % command)

    try:    
        retval = subprocess.check_output(cmd, shell=True)
        retval = re.sub('\n$', '', retval)
        return retval
    except:
        sys.exit("Unable to determine the path; halting.")
        return False


def readOpts():
    args= { 'board': 'stm32f407-disco-gcc',
            'binary': 'kubos-rt-example',
            'path': '/var/lib/ansible/'
            }

    try:
        print(str(sys.argv))
        print(str( len(sys.argv)))

    except:
        sys.exit("couldn't execute tests. Failing out.")

    try:
        if (sys.argv[3] is True):
            print("At least three arguments in the invocation: continuing")

    except AttributeError:
        print("WARNING: you must provide a path/filename and target board.")
        sys.exit("User did not provide required arguments.")

    except IndexError:
        print("%s inadequate number of program arguments." % errstr)
        sys.exit("User did not provide required arguments.")

    today=datetime.date.today()

    parser=argparse.ArgumentParser(description='''cistack.py; a python library
that provides a series of abstracted functions to interact with supported KubOS
target boards, through the Kubos Pi Hat v0.3 interface. The library provides
numerous functions, but chief among them is the ability to upload a compiled 
binary executable to each board using the flashing functions in the library.
As such, the user must provide, at a minimum, three arguments to any script that
calls this (readOpts) function. 

Example:

python yourscriptname.py -f mybinfile -p /path/to/binary -b boardtype -v

Supported boards include:
pyboard-gcc
stm32f407-disco-gcc (new-type boards)
msp430f5529-gcc
\n''')

# na-satbus-3c0-gcc
# kubos-msp430-gcc
# kubos-arm-none-eabi-gcc
# kubos-rt-gcc
# kubos-gcc
# stm32f405-gcc

    parser.add_argument("-f", "--file", action='store', \
        dest="inputbinary", default="kubos-rt-example",
        help="provide a filename for the compiled binary file to upload", metavar="FILE")

    parser.add_argument("-v", "--verbose", \
        action='store_true', dest="verbose",
        help="provide more verbose output to STDOUT", default=0)

# TODO add in support for this later.
#    parser.add_argument("-p", "--path", action='store', \
#        dest="binfilepath", default="/var/lib/ansible/",
#        help="provide a path to the binary you want to flash", metavar="PATH")

    parser.add_argument("-b", "--board", action='store', \
        dest="board", default="stm32f407-disco-gcc",
        help="the target board (and architecture) supported by the Kubos SDK", metavar="TARGET")
   
    arguments=parser.parse_args()
    args=vars(arguments)

    array=checkFile(args['inputbinary'])
    args['binfilepath']=array[0]
    args['inputbinary']=array[1]
#    print(args)

    checkBoard(args['board'])

    return args


def checkFile(binfile):
    '''Check input filename; split from absolute path if necessary.'''
    if (os.path.isfile(binfile) is False):
        sys.exit("%s Specified binary file doesn't appear to be a file.\n\
Exiting now." % errstr)

# (binpath, filename):
    array=os.path.split(binfile)

    if (os.path.exists(array[0]) is False):
        sys.exit("%s Unable to determine path to binary from input." % 
                errstr)

    return array

def checkBoard(board):
    '''Compare board name to list of currently supported boards.'''
    for b in supportedboards:
        if (b == board):
            return True
    errmsg=str("%s Board name '%s' does not match list of currently supported \
boards. Exiting." % (errstr, board))
    sys.exit(errmsg)
    return False

#####################################################################
# NOTE: these are placeholders at present.
#####################################################################


def sanityChecks(): # How do we check for executables in Windows?
    '''Check for dfu-util and other stuff.'''

    return False


def uploadSerial(filename):
    '''Send a binary file over the RPi UART.'''

    return False


#<EOF>
