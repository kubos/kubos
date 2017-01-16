import os
import sys
import magic
import re
import logging
import subprocess
from time import sleep
import RPi.GPIO as GPIO
from utils import findBin, whichUSBboard, getBoardConfigs
from target import Target
from pin import Pin

class NAsatbus(Target):
    """ Return an instance of Target specific to the NanoAvionics Satbus
    board actually requires a physical programmer, as it lacks a USB 
    interface to an integrated programmer. As such, it is quite different 
    physically from the other boards, but once hooked up to a physical 
    programming interface like some variant of the ST-LINK v2, it should 
    behave well. 
    Of note, the board can be programmed with any of the STM32F4 Discovery 
    board programmers (They are ST-Link V2-compatible).
    """

    def __init__(self):
        self.board = "na-satbus-3c0-gcc"
        self.arch = "ARM"
        self.cpu = "stm32f405"
        self.binfiletype = "ELF"
        self.pins = {
            'rst' : Pin(name = 'rst', number = 17),
            'pwr' : Pin(name = 'pwr', number = 27)
        }


# IMPORTANT NOTE: openocd must be version 0.9 or later.
    def flash(self, binobj):
        """
        Use an external shell to push the ELF file using openocd. It seems 
        to be necessary to pre-declare the LIB PATH for some commands, and 
        if the path variable is not available as declared in /etc/profile, it
        can be fixed here with the sp1 variable, below. HOWEVER: from ansible,
        the locally-declared and locally-requested path variables DO NOT WORK
        and cause ERRORS. Workaround: use the ansible -shell- command and 
        declare the library path before executing a bash -c command.
        
        IMPORTANT NOTE: openocd must be version 0.9 or later.
        """

      #  log = logging.getLogger('logfoo')
        logging.info("Initiating binary file flash.")

        if not self.sanitycheck(binobj):
            logging.error("Binary file didn't pass a sanity check.")
            return False

# TODO set all of these via Ansible, and get these vars from os.environ
        distpath = os.environ['KUBOS_LIB_PATH']
        configfiles = "../../flash/openocd"
        searchpath = str("%s/%s" % (distpath, configfiles))
        sp1 = os.environ['LD_LIBRARY_PATH']
        sp1 = str(sp1 + ":" + distpath)
        sp1 = str(sp1 + ":" + searchpath)

# will dfu-util work instead?

        openocdloc = findBin('openocd')
        unamestr = subprocess.check_output('uname')
        unamestr = re.sub('\n$', '', unamestr)

# TODO adjust the paths for OS X

# At present, this function only expects one board to be attached. TODO
        boards = whichUSBboard()
        configs = getBoardConfigs(boards)
        cfg = configs[2] # config file to use with openocd
        cmd = configs[3] # something like 'stm32f4_flash', an openocd command

        fileloc = binobj.abspath()
# $openocd -f $this_dir/$cfg -s $search_path -c "$cmd $file"
        command = str("%s -f %s/%s -s %s -c \"%s %s\"") % (openocdloc, 
            searchpath, cfg, searchpath, cmd, fileloc)
        logging.info("Attempting to flash the binary file to the target board.")
        logging.debug("Flashing the binary with:\n\n%s" % str(command))
        try:
            subprocess.check_output(command, shell = True)
            return True
        except:
            return False



#<EOF>
