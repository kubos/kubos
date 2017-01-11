import os
import sys
import magic
import re
import subprocess
from time import sleep
import RPi.GPIO as GPIO
from utils import *
from target import Target
from pin import Pin

class NAsatbus(Target):
# the NanoAvionics Satbus board actually requires a physical programmer,
# as it lacks a USB interface to an integrated programmer. As such, it is 
# quite different physically from the other boards, but once hooked up to
# a physical programming interface like some variant of the ST-LINK v2 
# it should behave well. 
# 
# At present, it works only when hooked up to an STM32f407 discovery 
# ST-Link V2 or V2-1 and not to an actual discrete ST-link v2 dongle
    def __init__(self):
        self.board = "na-satbus-3c0-gcc"
        self.arch = "ARM"
        self.cpu = "stm32f405"
        self.pins = {
            'rst' : Pin(name = 'rst', number = 17),
            'pwr' : Pin(name = 'pwr', number = 27)
#            'rst':[17, True, False, GPIO.OUT], # SWD connector, pull NRST to GND 
#            'prg':[18, True, False, GPIO.OUT], # none needed?
#            'pwr':[27, True, False, GPIO.OUT], # same as the rest of the hats
#            'opt':[22, True, False, GPIO.OUT]  # optional
        }



# IMPORTANT NOTE: openocd must be version 0.9 or later.
    def flash(self, binfile, binpath):
        """use an external shell to push the ELF file using openocd. It seems 
        to be necessary to pre-declare the LIB PATH for some commands, and 
        if the path variable is not available as declared in /etc/profile, it
        can be fixed here with the sp1 variable, below. HOWEVER: from ansible,
        the locally-declared and locally-requested path variables DO NOT WORK
        and cause ERRORS. Workaround: use the ansible -shell- command and 
        declare the library path before executing a bash -c command"""

# TODO set all of these via Ansible, and get these vars from os.environ
        distpath = "/usr/local/lib/python2.7/dist-packages/kubos"
        searchpath = str("%s/flash/openocd" % distpath)
        KUBOS_LIB_PATH = str("%s/lib/linux/" % distpath)
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
            subprocess.check_output(command, shell = True)
            return True
        except:
            return False


