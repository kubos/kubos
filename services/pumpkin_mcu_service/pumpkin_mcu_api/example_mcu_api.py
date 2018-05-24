#!/usr/bin/env python

# Copyright 2018 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.

"""
Example usage of the mcu_api
"""

import mcu_api

MODULES = mcu_api.CONFIG_DATA['modules']



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
        ERRORS = []
        for field in out:
            print (field,out[field])
            if out[field]['timestamp'] == 0:
                ERRORS.append([field])
        
        print "\n Errors: "
        for field in ERRORS:
            print (field,out[field])


module = "sim"
address = MODULES[module]['address']
fields = ["firmware_version","commands_parsed","scpi_errors","time"]
print('\nModule: ' + module)
print('Address: ' + str(address))
print('Fields: ' + str(fields) + '\n')
mcu = mcu_api.MCU(address = address)
out = mcu.read_telemetry(module = module,fields = fields)
ERRORS = []
for field in out:
    print (field,out[field])
    if out[field]['timestamp'] == 0:
        ERRORS.append([field])
