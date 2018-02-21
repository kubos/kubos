#!/usr/bin/env python

# Copyright 2017 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.

"""
API for interacting with all Pumpkin SupMCUs. 

See Pumpkin SUPERNOVA Firmware Reference Manual Rev 3.5
"""


import i2c

I2C_BUSNUM = 1

class MCU:
    
    def __init__(self,address):
        self.i2cfile = i2c.I2C(bus = I2C_BUSNUM)
        self.address = address
        
    def led(self,state):
        """
        Controls status LED
        """
        if state in ['OFF','ON','FLASH','ERROR','I2C']:
            self.send_cmd('SUP:LED '+state)

    def send_cmd(self,command):
        self.i2cfile.write(
            device = self.address,data = command.append('\x0A'))
            
    def read(self,count):
        readdict = {'timestamp': None,'data': None}
        if count < 6:
            raise TypeError('Read count must be at least 6 bytes.')
        data = self.i2cfile.read(device = self.address, count = count)
        if data[0] is not '\x01':
            raise IOError('Data is not ready')
        readdict['timestamp'] = int(data[1:5])
        readdict['data'] = data[5:]
        return readdict 
    
    def get_sup_telemetry(self):
        pass
        