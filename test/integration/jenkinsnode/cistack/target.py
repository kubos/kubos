import RPi.GPIO as GPIO
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

    def processor(self):
        return self.cpu

    def proc(self):
        return self.cpu

    def cpu(self):
        return self.cpu

    def progmode(self):
        return False

    def flash(self, binfile, binpath):
        return False

    def reset(self):
        """Assert the reset pin for the board, then release it."""
        self.pins['rst'].on()
        sleep(0.5)
        self.pins['rst'].off()
        sleep(0.5)
        return True

    def powerup(self):
        """Turn on the power MOSFET for the target board."""
        print ("Turning on the board with pin %s" % str(self.pins['pwr'].number))
        self.pins['pwr'].on()
        sleep(0.1)
        return True

    def powerdown(self):
        """Turn off the power MOSFET for the target board."""
        self.pins['pwr'].off()
        return True

    def setupboard(self):
        """Run this function immediately after determining which pins
        are assigned to your target board. This function sets the 
        Raspberry Pi GPIO map to "Broadcom" and then sets up pin 
        direction and function.
        """

# Setting BCM mode is "Broadcom", running from GPIO2 to GPIO27
# Meaning pin 40 in "BOARD" is pin 21 in BCM
        print("Setting pin modes for each pin:")
        GPIO.setmode(GPIO.BCM)

        for pin in self.pins.keys():
            print ("Setting up pin %s." % pin)
            self.pins[pin].setup()

        print("Setup successful.")
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

        binobj.validate()
        if (binobj.filetype == self.binfiletype and 
            binobj.arch == self.arch):
            return True
        else:
            return False


#<EOF>
