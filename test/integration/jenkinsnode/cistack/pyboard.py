import os
import sys
import magic
import re
import subprocess
import logging
from time import sleep
import RPi.GPIO as GPIO
from target import Target
from utils import *
from pin import Pin

class Pyboard(Target):
    def __init__(self):
        self.board = "pyboard-gcc"
        self.arch = "ARM"
        self.cpu = "stm32f405"
        self.binfiletype = "BIN"
        self.pins = {
            'rst' : Pin(name = 'rst', number = 17),
            'pwr' : Pin(name = 'pwr', number = 27),
            'prg' : Pin(name = 'prg', number = 18),
            'opt' : Pin(name = 'opt', number = 22)
        } 

    def progmode(self):
        """
        Assert two pins in sequence to enable programming mode on the 
        MicroPython board, then release them to reboot the board into
        programming mode.
        """
        self.pins['rst'].on()
        sleep(0.2)
        self.pins['prg'].on()
        sleep(0.5)
        self.pins['rst'].off()
        self.pins['prg'].off()
        sleep(0.1)
        return True

    def flash(self, binobj):
        """use an external shell to push the binary file using dfu-util."""

    #    log = logging.getLogger('logfoo')

        if not self.sanitycheck(binobj):
            logging.error("Binary file didn't pass a sanity check. Exiting.")
            return False

        dfupath = findBin('dfu-util')

# Note that the pyboard must be in programming mode for it to announce itself
# as a DFU device. That is, the system can't even find the board until it is
# in programming mode.
# Further, the USB interface requires 5VDC, even if the board does not.

        binfile = binobj.abspath()
        tail = str("-i 0 -s 0x08000000")
        head = str("--alt 0 -D ")
        command = str("%s %s %s %s " % (dfupath, head, binfile, tail))
        logging.info(command)
        try:
            output = subprocess.check_output(command , shell = True)
            logging.debug(output)

            if re.search("File downloaded successfully.*$", output):
                logging.info("Looks like things went well!")

        except:
            logging.error("Flash seems to have failed.")
            return False

        sleep(0.5)
        return True



#<EOF>
