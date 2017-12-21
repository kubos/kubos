import os
import sys
import re
import subprocess
import magic
import datetime
import argparse
import logging
from time import sleep
import RPi.GPIO as GPIO


errstr = "*** ERROR (functions):"

def supportedBoards():
    """Return a list of the supported boards."""
    supportedboards = [
    'pyboard-gcc',
    'stm32f407-disco-gcc',
    'msp430f5529-gcc', 
    'na-satbus-3c0-gcc'
    ]
    return supportedboards


def boardList():
    """Return a nicely formatted string of the supported boards."""
    boards = supportedBoards()
    boardlist = str("")
    for i in boards:
        boardlist = ("%s- %s\n" % (boardlist, i))
    return boardlist


def requiredUtils():
    """
    Return a list of the command line utilities required for this package.
    """
    requiredutils = [
    'dfu-util',
    'openocd',
    'mspdebug',
    'kubos',
    'lsusb',
    'gdb',
    'uname'
    ]
    return requiredutils


def requiredPaths():
    """
    Return a list of the paths that must be set/exported for this package
    to do the work required of it.
    """
    requiredpaths = [ 
    "KUBOS_LIB_PATH", 
    "LD_LIBRARY_PATH", 
    "PYTHONPATH", 
    "LIBUSB_LIBRARY"
    ]
    return requiredpaths


def getBinfile(name, path, board):
    from binfile import Binfile
    from basicbinfile import Basicbinfile
    """Create an instance of Binfile."""
    return Basicbinfile(name = name, path = path, board = board)


def getTarget(target):
    from nasatbus import NAsatbus
    from pyboard import Pyboard
    from stm32f407discovery import STM32F407Discovery
    from msp430f5529 import MSP430

#     log = logging.getLogger('logfoo') 

    """ Configure the board-specific pin addresses and directions """
    logging.info("Checking on the availability of the %s Target class" % target)

    if target == "pyboard-gcc":
        logging.debug("Matched STM32F405  pyboard")
        return Pyboard()

    elif target == "stm32f407-disco-gcc":
        logging.debug("Matched STM32F407 discovery board")
        return STM32F407Discovery()

    elif target == "msp430f5529-gcc":
        logging.debug("Matched MSP430F5529 launchpad")
        return MSP430()

    elif target == "na-satbus-3c0-gcc":
        logging.debug("Matched STM32F405 NanoAvionics SatBus 3c0")
        return NAsatbus()

    else:
        logging.error("Unsupported board -- no 'Target' class available.")
        return None

    return None


def getBoardConfigs(boards):
    """Ensure that the board identifier is supported."""
#    log = logging.getLogger('logfoo') 
    for i in boards:
        try:
            r = parseBoardIdentifier(i['dev'])
            if r[1]: # board is supported
                return r
        except:
            logging.error("Unable to determine board type. Exiting.")
            return None

    return None


def parseBoardIdentifier(lsusbpattern):
    """
    Parse the lsusb identifier assigned to the board. Note that some 
    boards such as the PyBoard won't be enumerated by lsusb unless they 
    are in DFU programming mode. Array values are as follows:
    key: lsusb identifier
    0: name of the device
    1: Is the board supported by Kubos?
    2: the configuration file for use by the flasher, if any
    3: the command or argument specific to the board (mostly for openocd)
    """

# FIXME
# But note that the STLINK-V2 could be connected to many different boards.
# Also: there's a v2 and a v2-1 config file for the STLINK programmer
    patterns = {
        '0483:3748': ['STMicro ST-LINK/V2 (old type)', 
            True, 'stm32f407vg.cfg', 'stm32f4_flash'],
        '0483:374b': ['STMicro ST-LINK/V2-1 (new type)', 
            True, 'stm32f407g-disc1.cfg', 'stm32f4_flash'],
        '0483:df11': ['STM32F405 PyBoard', 
            True, 'USE dfu-util!', '***'], 
        '0451:2046': ['TI MSP430F5529 Launchpad', 
            True, 'USE mspdebug!', '***'],
        '0451:f432': ['TI MSP430G2553 Launchpad', 
            False, 'NOT SUPPORTED', '/usr/bin/sleep 1']
    }

    if lsusbpattern in patterns:
        return patterns[lsusbpattern]

    return None


# kludgy at best, but helps. TODO replace with something better
def whichUSBboard():
#    log = logging.getLogger('logfoo') 
    lsusb = findBin('lsusb')
    if not lsusb:
        logging.error("Unable to find lsusb utility. Exiting.")
        return None
    output = subprocess.check_output(lsusb, shell = True)
    lines = output.rsplit('\n')
    retarray = []
    manlist = ['Texas', 'STMicro']

    for line in lines:
        arr = line.split(' ')
        for manuf in manlist:
            try:
                if re.search(manuf, arr[6]):
                    logging.info(str("found %s device at %s" % (manuf, arr[5])))
                    retarray.append({ 'manuf':arr[6], 'dev':arr[5]})
            except:
                next

# TODO add more board identifiers here
    return retarray


def findBin(command):
#    log = logging.getLogger('logfoo')
    cmd = str("/usr/bin/which %s" % command)
    logging.debug("Looking for %s in system binary PATHs." % command)

    try:    
        retval = subprocess.check_output(cmd, shell = True)
        retval = retval.rstrip()
        return retval
    except:
        sys.exit(str("Unable to determine the path to %s; halting." % 
            command))
        return None

def checkRoot():
    """
    If certain udev rules have not been set, it may be simpler to
    only allow the script to run with elevated privileges. While this
    choice is discouraged, it is up to the user's discretion.
    """
#    log = logging.getLogger('logfoo')
    logging.debug("Checking for root access...")

    if os.geteuid() != 0:
        print("You need to have root privileges to run this script.\n")
        sys.exit("Please try again, this time using 'sudo'. Exiting.")
        return False
    else:
        return True


def readOpts():
    """Read command line arguments and return Namespace object."""
    parser = argparse.ArgumentParser(description = helpstring)

    parser.add_argument("-r", "--root", action = 'store', \
        dest = "requireroot", default = False, help = "This script no longer \
requires root access to run, meaning you must implement \
a udev rule to enable user-land access to the device, or else just \
remember to sudo when running this script. \n\
\n\
Hint: ATTRS{idVendor}==\"0483\", ATTRS{idProduct}==\"df11\" \n\
\n\
Therefore, you can set this to True, but we don't advise it.", \
        required = False)

    parser.add_argument("-f", "--file", action = 'store', \
        dest = "inputbinary", default="kubos-rt-example", \
        help = "provide a filename for the compiled binary file to \
upload", metavar = "FILE", required = True)

    parser.add_argument("-v", "--verbose", \
        action = 'store_true', dest = "verbose",
        help = "set INFO log level", default = 0, \
        required = False)

    parser.add_argument("--debug", action = 'store_true', \
        dest = "debug", help="set DEBUG log level", default = 0, \
        required = False)

    parser.add_argument("--quiet", action = 'store_true', \
        dest = "quiet", help="set ERROR log level", default = 0, \
        required = False)

    parser.add_argument("-p", "--path", action = 'store', \
        dest = "binfilepath", default = "/var/lib/ansible/",
        help = "provide a path to the binary you want to flash", \
        metavar = "PATH", required = False)

    parser.add_argument("-b", "--board", action = 'store', \
        dest = "board", default = "stm32f407-disco-gcc",
        help = "the target board (and architecture) supported by the \
Kubos SDK", metavar = "TARGET", required = True)
   
    parser.add_argument("-i", "--ignore-warnings", action = 'store', \
        dest = 'ignoreGPIOwarnings', default = False, \
        help = "Ignore any warnings from the RPi.GPIO module. Not \
recommended.", required = False)

    parser.add_argument("--free-pins", action = 'store', \
        dest = 'freepins', default = False, \
        help = "Use RPi.GPIO module to Free GPIO pins when done. Usually \
unnecessary.", required = False)

    parser.add_argument("--shutdown", action = 'store', \
        dest = 'shutdown', default = False, \
        help = "Shut off the board when done. Usually unnecessary.",
        required = False)

    parser.add_argument("-c", "--command", action = 'store', \
        dest = 'command', default = "flash", type=str, \
        nargs='?', const='flash', \
        help = "Assign a functionality to the current session. \n\
        - 'flash' is default; \n\
        - 'lib' imports the file as a library; \n\
        - 'checks' runs tests on each of the flags and options.\n",\
        metavar = "COMMAND", required = False)

    arguments = parser.parse_args()

    return arguments

def logLevel(args):
    """Parse the command line arguments for log verbosity level."""
    if args.debug:
        return logging.DEBUG
    elif args.verbose:
        return logging.INFO
    elif args.quiet:
        return logging.ERROR
    else:
        return logging.WARNING

def checkBoard(board):
    """Compare board name to list of currently supported boards."""
#    log = logging.getLogger('logfoo') 
    supportedboards = supportedBoards()
    if board in supportedboards:
        return True
    logging.error(str("%s Board name '%s' does not match list of currently supported \
boards. Exiting." % (errstr, board)))
    return False

def allDone():
    """Free all the pins."""
    GPIO.cleanup()
    return True

def binaryChecks(findit):
    """
    Check for dfu-util, openocd, mspdebug, portions of the Kubos SDK,
    and other stuff.
    """
#    log = logging.getLogger('logfoo') 

    for i in findit:
        if not findBin(i):
            logging.error(str("ERROR: Unknown path to %s; halting." % command))
            return False

    return True

def startupChecks(args):
    """
    Check for dfu-util, openocd, mspdebug, portions of the Kubos SDK,
    and other stuff.
    """
#    log = logging.getLogger('logfoo') 
    

    logging.debug("Command line arguments are: %s " % str(args))
    if not args:
        logging.error("Command line arguments returned None. Exiting.")
        return False

    if not checkBoard(args.board):
        logging.error("Unable to verify or determine board. Exiting.")
        return None

    if args.requireroot:
        checkRoot()

    logging.debug("Checking for required system utilities.")
    requiredutils = requiredUtils()

    if binaryChecks(requiredutils):
        logging.info("Located required system utilities. Continuing.")
    else:
        logging.error("Unable to locate required utilites. Exiting.")
        return None

    paths = requiredPaths()
    if pathChecks(paths):
        logging.info("Located required system environment variables. Continuing.")
    else:
        logging.error("Unable to locate required environment variables. Exiting.")
        return None

    return True

def pathChecks(paths):
    """
    Check for the presence of system environment variables as submitted,
    but doesn't confirm anything is actually in the right place.
    """
    for i in paths:
        p = os.environ[i]
        try:
            logging.debug("Checking: %s  = %s" % (i,p))
            if ((p is None) or (p == "")):
                logging.error(str("No environment variable %s" % i))
                return False
        except:
            logging.error(str("No environment variable %s" % i))
            return False

    return True


def cleanUp(target, args):
    """
    When shutting the board down, this command cleans up and 
    de-energizes the power MOSFET.
    """
    if args.shutdown:
        logging.info("Shutting down the board.")
        target.powerdown() 

# If the args said to free the pins when done, do that.
    if args.freepins:
        logging.info("Freeing GPIO pins.")
        allDone()
   
# close up shop:
    dt = datetime.datetime
    NOW = dt.isoformat(dt.now(), '-')
    logging.debug("Script stopped at  %s." % str(NOW))
    sys.exit()
    return True # unnecessary but included pro forma



# Here's a global text variable.
boardlist = boardList()
global helpstring
helpstring = str("cistack.py; a python utility that provides a series \
of abstracted functions to interact with supported KubOS \
target boards, through the Kubos CISTACK Pi Hat interface. The library \
provides numerous functions, but chief among them is the ability to  \
upload a compiled binary executable to each board using the flashing \
functions in the library. As such, the user must provide, at a minimum, \
three arguments to any script that calls this (readOpts) function. \n\
\n\
Example:\n\
\n\
python cistack.py -f mybinfile -p /path/to/binary -b boardtype -v\n\
\n\
Supported boards include:\n\
\n\
%s\n\
\n" % boardlist)


#<EOF>
