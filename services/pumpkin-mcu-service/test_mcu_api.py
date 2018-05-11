#!/usr/bin/env python

# Copyright 2017 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.

"""
Testing mcu_api
"""

import mcu_api

print "\n Address 50: SIM"

sim_address = 0x50
sim = mcu_api.MCU(address = sim_address)
print sim.get_module_telemetry(module = 'sim')

print "\n Address 51: GPSRM"

gpsrm_address = 0x51
gpsrm = mcu_api.MCU(address = gpsrm_address)
print gpsrm.get_module_telemetry(module = 'gpsrm')

print "\n Address 53: PIM"

pim_address = 0x53
pim = mcu_api.MCU(address = pim_address)
print gpsrm.get_module_telemetry(module = 'pim')

# print "\n Address 54: DCPS"

# dcps_address = 0x54
# dcps = mcu_api.MCU(address = dcps_address)
# print dcps.get_sup_telemetry()

print "\n Address 55: RHM"

rhm_address = 0x55
rhm = mcu_api.MCU(address = rhm_address)
print gpsrm.get_module_telemetry(module = 'rhm')



