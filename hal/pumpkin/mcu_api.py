#!/usr/bin/env python

# Copyright 2017 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.

"""
API for interacting with all Pumpkin SupMCUs. 

See Pumpkin SUPERNOVA Firmware Reference Manual Rev 3.5
"""


import i2c,struct,time

I2C_BUSNUM = 1 # KubOS Default
DELAY = 0.300 # 300 millisecond delay between write and read
SUP_TELS = {
    'firmware_version': {
        'command':'SUP:TEL? 0,data',
        'length':53,
        'struct_arg':'s'
        },
    'commands_parsed': {
        'command':'SUP:TEL? 1,data',
        'length':13,
        'struct_arg':'<Q'
        },
    'scpi_errors': {
        'command':'SUP:TEL? 2,data',
        'length':13,
        'struct_arg':'<Q'
        },
    'voltstat': {
        'command':'SUP:TEL? 3,data',
        'length':13,
        'struct_arg':'<hhhh'
        },
    'cpu_selftests': {
        'command':'SUP:TEL? 4,data',
        'length':27,
        'struct_arg':'<QQhhh'
        },
    'time': {
        'command':'SUP:TEL? 5,data',
        'length':13,
        'struct_arg':'<Q'
        },
    'context_switches': {
        'command':'SUP:TEL? 6,data',
        'length':13,
        'struct_arg':'<Q'
        },
    'idling_hooks': {
        'command':'SUP:TEL? 7,data',
        'length':13,
        'struct_arg':'<Q'
        },
    'mcu_load': {
        'command':'SUP:TEL? 8,data',
        'length':13,
        'struct_arg':'<Q'
        },
    }

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
        timestamp = struct.unpack('<i',data[1:5])/100.0 # unpack timestamp in seconds
        data = data[5:] # telemetry data
        return {'timestamp':timestamp,'data':data} 
    
    def get_sup_telemetry(self):
        return self.get_telemetry(SUP_TELS)
    
    def get_telemetry(self,dict):
        output_dict = {}
        for telem_field in dict:
            self.write(dict[telem_field]['command'])
            time.sleep(DELAY)
            output_dict[telem_field] = self.read(
                dict[telem_field]['length']
                )
            output_dict[telem_field]['data'] = struct.unpack(
                dict[telem_field]['struct_arg'],
                output_dict[telem_field]['data']
                )
        return output_dict
            