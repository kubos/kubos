#!/usr/bin/env python

# Copyright 2018 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.

"""
Example usage of the mcu_api
"""

import mcu_api
import time

MODULES = {
    "sim":  {"address":80},
    "gpsrm":{"address":81},
    "aim2": {"address":0},
    "bim":  {"address":0},
    "pim":  {"address":83},
    "rhm":  {"address":85},
    "bsm":  {"address":0},
    "bm2":  {"address":92}
 }

# Sending commands
module = "sim"
address = MODULES[module]['address']
print('\nModule: ' + module)
print('Address: ' + str(address))
mcu = mcu_api.MCU(address = address)
turn_led_on_cmd = "SUP:LED ON"
turn_led_off_cmd = "SUP:LED OFF"
print mcu.write(turn_led_on_cmd)
time.sleep(3)
print mcu.write(turn_led_off_cmd)

# Read a selection of telemetry items
module = "sim"
address = MODULES[module]['address']
fields = ["firmware_version","commands_parsed","scpi_errors","time"]
print('\nModule: ' + module)
print('Address: ' + str(address))
print('Fields: ' + str(fields) + '\n')
mcu = mcu_api.MCU(address = address)
out = mcu.read_telemetry(module = module,fields = fields)
for field in out:
    print (field,out[field])


# Read all telemetry from all configured modules
for module in MODULES:
    module = str(module)
    address = MODULES[module]['address']
    if address == 0:
        print('\nModule not configured: ' + module + '\n')
    else:
        print('\nModule: ' + module)
        print('Address: ' + str(address) + '\n')
        mcu = mcu_api.MCU(address = address)
        out = mcu.read_telemetry(module = module)
        for field in out:
            print (field,out[field])



