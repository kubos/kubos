#!/usr/bin/env python

# Copyright 2017 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.

"""
API for interacting with all Pumpkin SupMCUs. 

See Pumpkin SUPERNOVA Firmware Reference Manual Rev 3.5
"""


import i2c,struct,time,json

with open('mcu_config.json') as config_file:
    data = json.load(config_file)

I2C_BUSNUM = data['i2c_bus_number']
DELAY = data['delay_between_writing_and_reading']
TELEMETRY = data['telemetry']
# json import converts to unicode, changing commands and parsing to be strings
for device in TELEMETRY:
    # print("device: ",device)
    for field in TELEMETRY[device]:
        # print('field: ', field)
        TELEMETRY[device][field]['command'] = str(TELEMETRY[device][field]['command'])
        TELEMETRY[device][field]['parsing'] = str(TELEMETRY[device][field]['parsing'])

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
        
    def get_module_telemetry(self,module,fields=None):
        if module not in TELEMETRY: 
            raise KeyError(
                'Module name: '+str(module)+' not found in mcu_config file.'
                )
        if type(fields) != list and fields != None: 
            raise TypeError(
                'fields argument must be a list of strings.'
                )
                
        module_telem = TELEMETRY[module]
        supervisor_telem = TELEMETRY['supervisor']
        if fields != None:
            for field in fields:
                if (
                    field not in module_telem or 
                    field not in supervisor_telem or 
                    type(field) != str
                    ):
                    raise ValueError('Invalid field: '+str(field))
                
        if fields is None:
            # Pulling all info
            requests = module_telem
            requests.update(supervisor_telem)
        elif fields is list:
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
        output_dict = self.__unpacking__(dict,output_dict)
        return output_dict
        
    def __unpacking__(self,input_dict,output_dict):
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