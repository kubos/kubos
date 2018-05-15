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
        print('\nModule not configured:' + module + '\n')
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
            
        
        
# print "\n Address 50: SIM"

# sim_address = 0x50
# sim = mcu_api.MCU(address = sim_address)
# out = sim.get_module_telemetry(module = 'sim')
# ERRORS = []
# for field in out:
#     print (field,out[field])
#     if out[field]['timestamp'] == 0:
#         ERRORS.append([field])

# print "\n Errors: "
# for field in ERRORS:
#     print (field,out[field])

# print "\n Address 51: GPSRM"

# gpsrm_address = 0x51
# gpsrm = mcu_api.MCU(address = gpsrm_address)
# out = gpsrm.get_module_telemetry(module = 'gpsrm')
# ERRORS = []
# for field in out:
#     print (field,out[field])
#     if out[field]['timestamp'] == 0:
#         ERRORS.append([field])

# print "\n Errors: "
# for field in ERRORS:
#     print (field,out[field])

# print "\n Address 53: PIM"

# pim_address = 0x53
# pim = mcu_api.MCU(address = pim_address)
# out = pim.get_module_telemetry(module = 'pim')
# ERRORS = []
# for field in out:
#     print (field,out[field])
#     if out[field]['timestamp'] == 0:
#         ERRORS.append([field])

# print "\n Errors: "
# for field in ERRORS:
#     print (field,out[field])
    
# # print "\n Address 54: DCPS"

# # dcps_address = 0x54
# # dcps = mcu_api.MCU(address = dcps_address)
# # print dcps.get_sup_telemetry()

# print "\n Address 55: RHM"

# rhm_address = 0x55
# rhm = mcu_api.MCU(address = rhm_address)
# out = rhm.get_module_telemetry(module = 'rhm')
# ERRORS = []
# for field in out:
#     print (field,out[field])
#     if out[field]['timestamp'] == 0:
#         ERRORS.append([field])

# print "\n Errors: "
# for field in ERRORS:
#     print (field,out[field])

# print "\n Address 5C: BM2"

# bm2_address = 0x5C
# bm2 = mcu_api.MCU(address = bm2_address)
# out = bm2.get_module_telemetry(module = 'bm2')
# ERRORS = []
# for field in out:
#     print (field,out[field])
#     if out[field]['timestamp'] == 0:
#         ERRORS.append([field])

# print "\n Errors: "
# for field in ERRORS:
#     print (field,out[field])