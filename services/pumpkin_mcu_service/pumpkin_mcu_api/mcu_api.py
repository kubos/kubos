#!/usr/bin/env python

# Copyright 2018 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.

"""
API for interacting with all Pumpkin SupMCUs. 

See Pumpkin SUPERNOVA Firmware Reference Manual Rev 3.5
"""


import struct,time,json
from i2c import i2c

# I know this is hacky...I need help with how to manage config files
import os.path
config_filename = 'mcu_config.json'
config_path = os.path.abspath(os.path.dirname(__file__)) + '/' + config_filename
with open(config_path) as config_file:
    CONFIG_DATA = json.load(config_file)

I2C_BUSNUM = CONFIG_DATA['i2c_bus_number']
DELAY = CONFIG_DATA['delay_between_writing_and_reading']
TELEMETRY = CONFIG_DATA['telemetry']
HEADER_SIZE = CONFIG_DATA['header_size']
# json import converts to unicode, changing type sensitive fields to be strings
for device in TELEMETRY:
    # print("device: ",device)
    for field in TELEMETRY[device]:
        # print('field: ', field)
        TELEMETRY[device][field]['command'] = str(TELEMETRY[device][field]['command'])
        TELEMETRY[device][field]['parsing'] = str(TELEMETRY[device][field]['parsing'])
        TELEMETRY[device][field]['length'] = TELEMETRY[device][field]['length'] + HEADER_SIZE

class MCU:
    
    def __init__(self,address):
        self.i2cfile = i2c.I2C(bus = I2C_BUSNUM)
        self.address = address

    def write(self,command):
        """
        Write command used to append the proper stopbyte to all writes.
        """
        if type(command) is str:
            return self.i2cfile.write(
                device = self.address,data = command+'\x0A')
        else:raise TypeError('Commands must be strings.')    
    
    def read(self,count):
        if count < (HEADER_SIZE+1):
            raise TypeError('Read count must be at least '+str(HEADER_SIZE+1)+' bytes.')
        data = raw_read(count = count)
        if data[0] != '\x01':
            #raise IOError('Data is not ready')
            return {'timestamp':0,'data':(count-HEADER_SIZE)*'\xff'}
        timestamp = struct.unpack('<i',data[1:HEADER_SIZE])[0]/100.0 # unpack timestamp in seconds
        data = data[HEADER_SIZE:] # telemetry data
        return {'timestamp':timestamp,'data':data} 
    
    def raw_read(self,count):
        return self.i2cfile.read(device = self.address, count = count)
    
    def get_sup_telemetry(self,fields=["all"]):
        if fields == ["all"]:
            requests = TELEMETRY['supervisor']
        elif fields is list:
            requests = {}
            for field in fields:
                if field not in TELEMETRY['supervisor'] or type(field) != str:
                    raise ValueError(
                        'Invalid field: '+str(field))
                else:
                    requests[field] = TELEMETRY['supervisor'][field]
        else:
            raise TypeError('fields must be a list of strings.')
        
        return self.get_telemetry(requests)
        
    def get_module_telemetry(self,module,fields=["all"]):
        if module not in TELEMETRY: 
            raise KeyError(
                'Module name: '+str(module)+' not found in mcu_config file.'
                )
        if type(fields) != list: 
            raise TypeError(
                'fields argument must be a list of strings.'
                )
                
        module_telem = TELEMETRY[module]
        supervisor_telem = TELEMETRY['supervisor']
        if fields != ["all"]:
            for field in fields:
                if field not in module_telem or \
                   field not in supervisor_telem or \
                   type(field) != str:
                    raise ValueError('Invalid field: '+str(field))
                
        if fields == ["all"]:
            # Pulling all info
            requests = module_telem
            requests.update(supervisor_telem)
        else:
            requests = {}
            for field in fields:
                if field in module_telem:
                    requests[field] = module_telem[field]
                if field in supervisor_telem:
                    requests[field] = supervisor_telem[field]
            
        return self.get_telemetry(requests)
        
    def get_telemetry(self,dict):
        output_dict = {}
        for telem_field in dict:
            self.write(dict[telem_field]['command'])
            time.sleep(DELAY)
            output_dict[telem_field] = self.read(
                dict[telem_field]['length']
                )
        output_dict = self._unpacking(dict,output_dict)
        return output_dict
        
    def _unpacking(self,input_dict,output_dict):
        for field in input_dict:
            parsing = input_dict[field]["parsing"]
            if parsing == "s":
                # Leave strings alone
                pass
            elif (len(parsing) == 2 and parsing[0] in ['<','>']) or len(parsing) == 1:
                # Only one value, parse and store
                output_dict[field]['data'] = struct.unpack(parsing,output_dict[field]["data"])[0]
            else:
                # Multiple values. Parse and store as new fields
                names = input_dict[field]["names"]
                parsed_values = struct.unpack(parsing,output_dict[field]["data"])
                if len(parsed_values) == len(names):
                    for new_field in names:
                        output_dict[new_field] = {
                            # Store parsed telemetry in output with corresponding field name
                            "data":parsed_values[names.index(new_field)],
                            # Copy timestamp
                            "timestamp":output_dict[field]["timestamp"] 
                        }
        return output_dict