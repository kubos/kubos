#!/usr/bin/env python3

# Copyright 2018 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.

"""
Example usage of the mcu_api
"""

import mcu_api
import time

MODULES = {
    "sim":  {"address": 80},
    "gpsrm": {"address": 81},
    "pim":  {"address": 83},
    "rhm":  {"address": 85},
    "bm2":  {"address": 92}
}

# Sending commands
module = "sim"
address = MODULES[module]['address']
print('\nModule: {}'.format(module))
print('Address: {}'.format(str(address)))
mcu = mcu_api.MCU(address=address)
turn_led_on_cmd = "SUP:LED ON"
turn_led_off_cmd = "SUP:LED OFF"
print(mcu.write(turn_led_on_cmd))
time.sleep(3)
print(mcu.write(turn_led_off_cmd))

# Read a selection of telemetry items
module = "sim"
address = MODULES[module]['address']
fields = ["firmware_version", "commands_parsed", "scpi_errors", "time"]
print('\nModule: {}'.format(module))
print('Address: {}'.format(str(address)))
print('Fields: {}\n'.format(str(fields)))
mcu = mcu_api.MCU(address=address)
out = mcu.read_telemetry(module=module, fields=fields)
for field in out:
    print("{}{}".format(field, out[field]))


# Read all telemetry from all modules
for module in MODULES:
    module = str(module)
    address = MODULES[module]['address']
    print('\nModule: {}'.format(module))
    print('Address: {}\n'.format(str(address)))
    mcu = mcu_api.MCU(address=address)
    out = mcu.read_telemetry(module=module)
    for field in out:
        print("{}{}".format(field, out[field]))
