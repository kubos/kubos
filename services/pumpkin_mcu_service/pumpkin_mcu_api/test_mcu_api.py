#!/usr/bin/env python

# Copyright 2017 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.

"""
Testing mcu_api
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
        out = mcu.get_module_telemetry(module = module)
        ERRORS = []
        for field in out:
            print (field,out[field])
            if out[field]['timestamp'] == 0:
                ERRORS.append([field])
        
        print "\n Errors: "
        for field in ERRORS:
            print (field,out[field])