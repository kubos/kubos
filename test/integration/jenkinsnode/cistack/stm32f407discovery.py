import os
import sys
import magic
import re
import subprocess
import logging
from time import sleep
import RPi.GPIO as GPIO
from utils import findBin, whichUSBboard, getBoardConfigs
from target import Target
from pin import Pin


class STM32F407Discovery(Target):
    def __init__(self):
        self.board = "stm32f407-disco-gcc"
        self.arch = "ARM"
        self.cpu = "stm32f407"
        self.binfiletype = "ELF"
        self.pins = {
            'rst' : Pin(name = 'rst', number = 17),
            'pwr' : Pin(name = 'pwr', number = 27),
            'opt' : Pin(name = 'opt', number = 22)  # optional
        }

# IMPORTANT NOTE: openocd must be version 0.9 or later.
    def flash(self, binobj):
        from utils import findBin, whichUSBboard, getBoardConfigs
        """
        Use an external shell to push the ELF file using openocd. It seems 
        to be necessary to pre-declare the LIB PATH for some commands, and 
        if the path variable is not available as declared in /etc/profile, it
        can be fixed here with the sp1 variable, below. HOWEVER: from ansible,
        the locally-declared and locally-requested path variables DO NOT WORK
        and cause ERRORS. Workaround: use the ansible -shell- command and 
        declare the library path before executing a bash -c command.
        """

  #      log = logging.getLogger('logfoo')
# Sanity check the arch and bin file type:
        if not self.sanitycheck(binobj):
            logging.error("Binary file didn't pass a sanity check. Exiting.")
            return False

        if not self.checkflasher():
            logging.error("Binary file was not current enough. Exiting.")
            return False

# TODO set all of these via Ansible on the target machines
        distpath = os.environ['KUBOS_LIB_PATH']
        configfiles = "../../flash/openocd"
        searchpath = str("%s/%s" % (distpath, configfiles))
        sp1 = os.environ['LD_LIBRARY_PATH']
        sp1 = str(sp1 + ":" + distpath)
        sp1 = str(sp1 + ":" + searchpath)

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
        logging.info(str(command))
        try:
            subprocess.check_output(command, shell = True)
            return True
        except:
            return False

    def checkflasher(self):
        openocdloc = findBin('openocd')
        command = str("%s --version " % openocdloc)
        versionlines = subprocess.check_output(command, shell=True, stderr=subprocess.STDOUT) 
        logging.debug(str("%s" % versionlines))
        for i in versionlines:
            if re.search("Open On-Chip Debugger\s+0\.[12345678]", i):
                logging.critical("OpenOCD must be version 0.9 or higher.")
                return False
        logging.info("OpenOCD seems to be version 0.9 or higher.")
        return True

#<EOF>
