import os
import sys
import magic
import re
import subprocess
from time import sleep
import RPi.GPIO as GPIO
from target import Target
from pin import Pin

errstr = "*** ERROR (msp430 target) "

class MSP430(Target):
    def __init__(self):
        self.board = "msp430f5529-gcc"
        self.arch = "MSP430"
        self.cpu = "msp430f5529"
        self.binfiletype = "ELF"
        self.pins = {
            'rst' : Pin(name = 'rst', number = 17),
            'pwr' : Pin(name = 'pwr', number = 27),
            'opt' : Pin(name = 'opt', number = 22)  # optional
        }
    
    def flash(self, binobj):
        from utils import findBin
        """Flash a binary file through USB connection on an MSP430 Launchpad"""

        if not self.sanitycheck(binobj):
            sys.exit("Binary file didn't pass a sanity check. Exiting.")
            return False

        searchpath = os.environ['KUBOS_LIB_PATH']
        sp1 = os.environ['LD_LIBRARY_PATH']
        sp1 = str(sp1 + ":" + searchpath)

        fileloc = binobj.abspath()
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


#<EOF>
