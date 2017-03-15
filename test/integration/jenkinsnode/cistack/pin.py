import os
import sys
import re
import RPi.GPIO as GPIO
import logging
from time import sleep

errstr="*** ERROR (pin) "

class Pin(object):

    def __init__(self, name = "", number = "", \
        direction = GPIO.OUT, onval = True, offval = False, \
        pullup = None, pulldown = None, mux = None):
        
        self.name = name
        self.number = number
        self.direction = direction
        self.onval = onval
        self.offval = offval
        self.pullup = pullup
        self.pulldown = pulldown
        self.mux = mux

    def number(self):
        return self.number

    def name(self):
        return self.name

    def onval(self):
        return self.onval

    def offval(self):
        return self.offval

    def direction(self):
        return self.direction

    def setup(self):
        """Set up one GPIO pin per the pin dict values. """
        
        GPIO.setmode(GPIO.BCM)

        if self.number is False:
            return False
        if self.number is None:
            return False
        if self.direction != GPIO.OUT:
            return False

        func = GPIO.gpio_function(self.number)
        logging.debug("pin %s set to %s" % (str(self.number), str(func)))
        if func == GPIO.UNKNOWN:
            sys.exit("Unable to determine the function of the pin! Exiting.")

# Can attempt to set this using incorrect options, but otherwise it isn't
# particularly interactive
        GPIO.setup(self.number, self.direction)
        logging.info("Key %s, pin %s is set to %s " % 
        (self.name, str(self.number), str(self.direction)))
        sleep(0.5)
        return True

    def on(self):
        """Generic "assert the GPIO pin" function."""

        if self.number is None:
            return False

# GPIO.output *will* throw exceptions but so far just the RuntimeError.
# It is possible to check beforehand and avoid the Error, but that might do
# things the user didn't intend.
        try:
            logging.debug("Asserting pin %s" % str(self.number))
            GPIO.output(self.number, self.onval)
        
        except RuntimeError:
            logging.error("%s in pin %s:" % (errstr, str(self.number)))
            logging.error("onval: %s" % str(self.onval))
            logging.error("offval: %s" % str(self.offval))
            logging.error("direction: %s " % str(self.direction))

            sys.exit("%s unable to assert pin %s" % (errstr, str(self.number)))
            return False

        except:
            return False

        return True


    def off(self):
        """Generic "turn off the GPIO pin" function."""
    
        if self.number is None:
            return False
        try:
            GPIO.output(self.number, self.offval)

        except RuntimeError:
            sys.exit("%s unable to change pin %s" % (errstr, str(self.number)))
            return False

        except:
            return False

        return True

#<EOF>
