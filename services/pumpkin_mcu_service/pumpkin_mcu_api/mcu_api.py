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
        return self.i2cfile.read(device = self.address, count = count)
        
    def read_telemetry(self,module,fields=["all"]):
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
                if (field not in module_telem and 
                   field not in supervisor_telem) or \
                   type(field) not in [str,unicode]:
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
            
        return self._read_telemetry_items(requests)
        
    def _read_telemetry_items(self,dict):
        """
        Creates the output_dict, reads the data, then inserts and formats it 
        in the output_dict. 
        """
        # Create empty dictionary
        output_dict = {}
        for telem_field in dict:
            input_dict = dict[telem_field]
            # Write command for the MCU to prepare the data
            self.write(input_dict['command'])
            # Delay time specified in the Supervisor Reference Manual
            time.sleep(DELAY)
            # Read the data, check and parse the header
            read_data = self._header_read(input_dict['length'])
            # Parse the data
            parsed_data = self._unpack(
                data = read_data['data'],
                parsing = parsing)
            if len(parsed) > 1:
                # Multiple items parsed
                if "names" not in input_dict:
                    raise KeyError("Must be a names field for parsing multiple \
                        items")
                if len(input_dict['names']) != len(parsed_data):
                    raise KeyError("Number of field names doesn't match parsing\
                        strings")
                for ind,field in enumerate(input_dict['names']):
                    output_dict.update(
                        {field: {
                            'timestamp':read_data['timestamp'],
                            'data':parsed_data[ind]})
            else:
                # Single item parsed - pull in dict then update with parsed data
                output_dict[telem_field] = read_data
                output_dict[telem_field]['data'] = parsed_data[0]
        
        return output_dict
    
    def _header_read(self,count):
        """
        Reads and parses the header data. Format is:
        [data ready flag][timestamp][data]
        output format is:
        {'timestamp':timestamp,'data':data}
        """
        if count < (HEADER_SIZE+1):
            # There must be at least 1 byte of data. 
            raise TypeError('Read count must be at least '+
                str(HEADER_SIZE+1)+' bytes.')
        data = self.read(count = count)
        if data[0] != '\x01':
            # Returns 0 for timestamp if data was not ready, but still returns
            # the data for debugging purposes.
            return {'timestamp':0,'data':data[HEADER_SIZE:] # telemetry data}
        
        # Unpack timestamp in seconds. 
        timestamp = struct.unpack('<i',data[1:HEADER_SIZE])[0]/100.0 
        # Return the valid packet timestamp and data
        return {'timestamp':timestamp,'data':data[HEADER_SIZE:]} 
        
    def _unpack(self,data,parsing):
        if type(parsing) not in [str,unicode]:
            # Check that parsing is a valid type
            raise TypeError('Parsing field must be a valid struct parsing \
            string. Input: '+str(type(parsing)))
        
        if parsing == "s":
            # Leave strings alone
            return (data)
        elif parsing == "h":
            # store as a hex string. This is so we can return binary data
            return (data.encode('hex'))
        
        # All others parse directly with the parsing string. 
        return struct.unpack(parsing,data)
        
        
    def _unpacking(self,input_dict,output_dict):
        """
        Takes in a dictionary of data from I2C reads and parses it into single
        or multiple fields according to the string in the parsing field. String
        must be a struct package format string. 
        """
        for field in input_dict:
            parsing = input_dict[field]["parsing"]
            if parsing == "s":
                # Leave strings alone
                pass
            elif parsing == "h":
                # store as a hex string. This is so we can return binary data
                output_dict[field]['data'] = \
                    output_dict[field]['data'].encode('hex')
            elif (len(parsing) == 2 and parsing[0] in ['<','>']) or \
                len(parsing) == 1:
                # Only one value, parse and store
                output_dict[field]['data'] = \
                    struct.unpack(parsing,output_dict[field]["data"])[0]
            else:
                # Multiple values. Parse and store as new fields
                names = input_dict[field]["names"]
                parsed_values = struct.unpack(
                    parsing,output_dict[field]["data"])
                if len(parsed_values) == len(names):
                    for new_field in names:
                        output_dict[new_field] = {
                            # Store parsed telemetry in output 
                            # with corresponding field name
                            "data":parsed_values[names.index(new_field)],
                            # Copy timestamp
                            "timestamp":output_dict[field]["timestamp"] 
                        }
        return output_dict