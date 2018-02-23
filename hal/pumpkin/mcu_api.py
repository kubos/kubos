#!/usr/bin/env python

# Copyright 2017 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.

"""
API for interacting with all Pumpkin SupMCUs. 

See Pumpkin SUPERNOVA Firmware Reference Manual Rev 3.5
"""


import i2c,struct,time,json

with open('config.json') as config_file:
    data = json.load(config_file)

I2C_BUSNUM = data['i2c_bus_number']
DELAY = data['delay_between_writing_and_reading']
SUP_TELS = data['available_telemetry']

class MCU:
    
    def __init__(self,address):
        self.i2cfile = i2c.I2C(bus = I2C_BUSNUM)
        self.address = address
        
    def led(self,state):
        """
        Controls status LED
        """
        if state in ['OFF','ON','FLASH','ERROR','I2C']:
            self.write('SUP:LED '+state)

    def write(self,command):
        """
        Write command used to append the proper stopbyte to all writes.
        """
        if type(command) is str:
            self.i2cfile.write(
                device = self.address,data = command+'\x0A')
        else:raise TypeError('Commands must be strings.')    
    
    def read(self,count):
        if count < 6:
            raise TypeError('Read count must be at least 6 bytes.')
        data = self.i2cfile.read(device = self.address, count = count)
        if data[0] != '\x01':
            raise IOError('Data is not ready')
        timestamp = struct.unpack('<i',data[1:5])[0]/100.0 # unpack timestamp in seconds
        data = data[5:] # telemetry data
        return {'timestamp':timestamp,'data':data} 
    
    def get_sup_telemetry(self,fields=None):
        if fields is None:
            requests = SUP_TELS
        elif fields is list:
            requests = {}
            for field in fields:
                if field not in SUP_TELS or type(field) != str:
                    raise ValueError(
                        'Invalid field: '+str(field))
                else:
                    requests[field] = SUP_TELS[field]
        else:
            raise TypeError('fields must be a list of strings.')
        
        return self.get_telemetry(requests)
    
    def get_telemetry(self,dict):
        output_dict = {}
        for telem_field in dict:
            self.write(dict[telem_field]['command'])
            time.sleep(DELAY)
            output_dict[telem_field] = self.read(
                dict[telem_field]['length']
                )
        return output_dict
            