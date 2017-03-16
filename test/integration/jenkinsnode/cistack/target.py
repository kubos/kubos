import RPi.GPIO as GPIO
import logging
from utils import * 

class Target(object):

    def __init__(self):
        self.board = ""
        self.arch = ""
        self.cpu = ""
        self.binfiletype = ""
        self.pins = {}

    def getboard(self):
        return self.board

    def getpins(self):
        return self.pins

    def arch(self):
        return self.arch

    def cpu(self):
        return self.cpu

    def progmode(self):
        """
        Assert the pin(s) in whatever sequence is required to
        put the board into programming mode, if this is required.
        """
        logging.debug("Setting programming mode, if any is needed.")
        return False

    def flash(self, binfile, binpath):
        return False

    def reset(self):
        """Assert the reset pin for the board, then release it."""
        logging.debug("Resetting the board.")
        self.pins['rst'].on()
        sleep(0.5)
        self.pins['rst'].off()
        sleep(0.5)
        return self.pins['rst'].off()

    def powerup(self):
        """Turn on the power MOSFET for the target board."""
        logging.debug("Turning on the board with pin %s" % str(self.pins['pwr'].number))
        return self.pins['pwr'].on()

    def powerdown(self):
        """Turn off the power MOSFET for the target board."""
        logging.debug("Turning off the board's power")
        return self.pins['pwr'].off()

    def setupboard(self):
        """Run this function immediately after determining which pins
        are assigned to your target board. This function sets the 
        Raspberry Pi GPIO map to "Broadcom" and then sets up pin 
        direction and function.
        """

# Setting BCM mode is "Broadcom", running from GPIO2 to GPIO27
# Meaning pin 40 in "BOARD" is pin 21 in BCM
        logging.info("Setting pin modes for each pin:")
        GPIO.setmode(GPIO.BCM)

        for pin in self.pins.keys():
            logging.debug("Setting up pin %s." % pin)
            self.pins[pin].setup()

        logging.info("Setup complete.")
        sleep(0.1)
        return True


    def sanitycheck(self, binobj):
        """
        Ensure that the binary file to be flashed matches the filetype 
        specified for each board type. It seems that .elf files know 
        where to go, because of the debugging information; bin files 
        lack that information and have to be told, among other things,
        where in the memory space to start putting the binary. One 
        problem is that .elf files usually don't have file name suffixes, 
        meaning it cannot be simply found with a regex.
        """

# do self.binfiletype & self.arch match returns from binfile.validate()?
        logging.debug("Validating the binary file object")
        binobj.validate()
        
        logging.debug("comparing %s to %s and %s to %s" % 
                (str(binobj.filetype), str(self.binfiletype), 
                    str(binobj.arch), str(self.arch)))
        if (binobj.filetype == self.binfiletype and 
            binobj.arch == self.arch):
            return True
        else:
            return False


#<EOF>
