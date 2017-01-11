import RPi.GPIO as GPIO
from utils import * 

class Target(object):

    def __init__(self):
        self.board = ""
        self.arch = ""
        self.cpu = ""
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
#        pinOn('rst', **self.pins)
        self.pins['rst'].on() 
        sleep(0.5)
#        pinOff('rst', **self.pins)
        self.pins['rst'].off()
        sleep(0.5)
        return True

    def powerup(self):
        """Turn on the power MOSFET for the target board."""
#        pinOn('pwr', **self.pins)
        print ("Turning on the board with pin %s" % str(self.pins['pwr'].number))
        self.pins['pwr'].on()
        sleep(0.1)
        return True

    def powerdown(self):
        """Turn off the power MOSFET for the target board."""
#        pinOff('pwr', **self.pins)
        self.pins['pwr'].off()
        return True

    def setupboard(self):
        """Run this function immediately after determining which pins
are assigned to your target board. This function sets the Raspberry
Pi GPIO map to "Broadcom" and then sets up pin direction and function."""

# Setting BCM mode is "Broadcom", running from GPIO2 to GPIO27
# Meaning pin 40 in "BOARD" is pin 21 in BCM
        print("Setting pin modes for each pin:")
        GPIO.setmode(GPIO.BCM)

        for pin in self.pins.keys():
#            pinSetup(pin, **self.pins)
            print ("Setting up pin %s." % pin)
            self.pins[pin].setup()

        print("Setup successful.")
        sleep(0.1)
        return True

