#!/usr/bin/env python

# Copyright 2018 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.

"""
API for interacting with all Pumpkin SupMCUs. 

See Pumpkin SUPERNOVA Firmware Reference Manual Rev 3.5
"""


import struct,time,json
import i2c

# I know this is hacky...I need help with how to manage config files
import os.path
config_filename = 'mcu_config.json'
config_path = os.path.abspath(os.path.dirname(__file__)) + '/' + config_filename
with open(config_path) as config_file:
    CONFIG_DATA = json.load(config_file)

# Pull all the config variables from the config file. 
I2C_BUSNUM = CONFIG_DATA['i2c_bus_number']
DELAY = CONFIG_DATA['delay_between_writing_and_reading']
TELEMETRY = CONFIG_DATA['telemetry']
HEADER_SIZE = CONFIG_DATA['header_size']
# json import converts to unicode, changing type sensitive fields to be strings
for device in TELEMETRY:
    for field in TELEMETRY[device]:
        TELEMETRY[device][field]['command'] = str(TELEMETRY[device][field]['command'])
        TELEMETRY[device][field]['parsing'] = str(TELEMETRY[device][field]['parsing'])
        TELEMETRY[device][field]['length'] = TELEMETRY[device][field]['length'] + HEADER_SIZE

class MCU:
    
    def __init__(self,address):
        """
        Sets the bus number and stores the address
        """
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
        """
        Read and parse specific fields from the MCUs that are contained in the
        config file. 
        
        Input: 
        module = string module name. Must exactly match the module name in the 
        config file and the I2C address must be valid and non-zero. If address
        is 0, it assumes the module is not present/not configured.
        fields = list of strings, strings must exactly match fields in 
        the config file listed in the "telemetry" section under "supervisor" or
        the specific module name. If field is left blank it defaults to ["all"],
        which pulls all available telemetry for that module. 
        
        Output: A dict with keys for all fields requested with "timestamp" and
        "data" keys for each field. 
        
        This method specifically just builds the dictionary of requested data
        and passes it into the method that does the actual reading. 
        """
        if module not in TELEMETRY: 
            # Check that module is listed in config file
            raise KeyError(
                'Module name: '+str(module)+' not found in mcu_config file.')
        if type(fields) != list: 
            # Validate fields input type
            raise TypeError(
                'fields argument must be a list of strings.')
                
        module_telem = TELEMETRY[module]
        supervisor_telem = TELEMETRY['supervisor']
        if fields == ["all"]:
            # Pulling all info
            requests = module_telem
            requests.update(supervisor_telem)
            return self._read_telemetry_items(requests)
        
        # Builds requested dict
        # Validates fields input values
        requests = {}
        for field in fields:
            if field in module_telem:
                requests[field] = module_telem[field]
            elif field in supervisor_telem:
                requests[field] = supervisor_telem[field]
            else:
                raise ValueError('Invalid field: '+str(field))
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
            read_data = self._header_parse(input_dict['length'])
            # Parse the data
            parsed_data = self._unpack(
                parsing = input_dict['parsing'],
                data = read_data['data'])
            if len(parsed_data) > 1:
                # Multiple items parsed
                if "names" not in input_dict:
                    raise KeyError(
                        "Must be a names field for parsing multiple items")
                if len(input_dict['names']) != len(parsed_data):
                    raise KeyError(
                        "Number of field names doesn't match parsing strings")
                for ind,field in enumerate(input_dict['names']):
                    output_dict.update(
                        {field: {
                            'timestamp':read_data['timestamp'],
                            'data':parsed_data[ind]}})
            else:
                # Single item parsed - pull in dict then update with parsed data
                output_dict[telem_field] = read_data
                output_dict[telem_field]['data'] = parsed_data[0]
        
        return output_dict
    
    def _header_parse(self,count):
        """
        Reads and parses the header data. Format is:
        [data ready flag][timestamp][data]
        output format is:
        {'timestamp':timestamp,'data':data}
        """
        if type(count) != int:
            # Validate count type
            raise TypeError("count must be an int. Type given: "\
                +str(type(count)))
        if count < (HEADER_SIZE+1):
            # Check count value
            # There must be at least 1 byte of data. 
            raise TypeError('Read count must be at least '+
                str(HEADER_SIZE+1)+' bytes.')
        data = self.read(count = count)
        if data[0] != '\x01':
            # Returns 0 for timestamp if data was not ready, but still returns
            # the data for debugging purposes.
            return {'timestamp':0,'data':data[HEADER_SIZE:]} # telemetry data}
        
        # Unpack timestamp in seconds. 
        timestamp = struct.unpack('<i',data[1:HEADER_SIZE])[0]/100.0 
        # Return the valid packet timestamp and data
        return {'timestamp':timestamp,'data':data[HEADER_SIZE:]} 
        
    def _unpack(self,parsing,data):
        """
        Basically just an abstraction of struct.unpack() to allow for types that
        are not standard in the method. 
        
        Input data read over I2C from a Pumpkin module and parsing string that
        indicates a special parsing method or is a valid format string for the
        python struct.unpack() method. 
        
        Outputs a tuple where each field is an item parsed. 
        """
        if type(parsing) not in [str,unicode]:
            # Check that parsing is a valid type
            raise TypeError(
                'Parsing field must be a valid struct parsing string. Input: '\
                +str(type(parsing)))
        
        if parsing == "str":
            # Leave strings alone, return it in a tuple
            return (data,)
        elif parsing == "hex":
            # Store as a hex string. This is so we can return binary data.
            # Return as a single field in a tuple
            return (data.encode('hex'),)
        
        # All others parse directly with the parsing string. 
        return struct.unpack(parsing,data)