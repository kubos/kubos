#!/usr/bin/env python

# Copyright 2018 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.

"""
API for interacting with all Pumpkin SupMCUs. 

See Pumpkin SUPERNOVA Firmware Reference Manual Rev 3.5
"""


import struct,time
import i2c

#############
# Config Data
DELAY = 0.200
I2C_BUS_NUM = 1
HEADER_SIZE = 5
TELEMETRY = {
    "supervisor":{
        "firmware_version": {"command":"SUP:TEL? 0,data","length":48,"parsing":"str"},
        "commands_parsed":  {"command":"SUP:TEL? 1,data","length":8, "parsing":"<Q"},
        "scpi_errors":      {"command":"SUP:TEL? 2,data","length":8, "parsing":"<Q"},
        "voltstat":         {"command":"SUP:TEL? 3,data","length":8, "parsing":"<hhhh",
            "names":["voltstat0","voltstat1","voltstat2","voltstat3"]},
        "cpu_selftests":    {"command":"SUP:TEL? 4,data","length":22,"parsing":"<QQhhh",
            "names":["selftest0","selftest1","selftest2","selftest3","selftest4"]},
        "time":             {"command":"SUP:TEL? 5,data","length":8, "parsing":"<Q"},
        "context_switches": {"command":"SUP:TEL? 6,data","length":8, "parsing":"<Q"},
        "idling_hooks":     {"command":"SUP:TEL? 7,data","length":8, "parsing":"<Q"},
        "mcu_load":         {"command":"SUP:TEL? 8,data","length":8, "parsing":"<Q"}
    },
    "sim":{},
    "gpsrm":{
        "status":           {"command":"GPS:TEL? 0,data","length":2,  "parsing":"hex"},
        "nmea_string":      {"command":"GPS:TEL? 1,data","length":512,"parsing":"str"},
        "propagator":       {"command":"GPS:TEL? 2,data","length":56, "parsing":"<ddddddd",
            "names":["propagator0","propagator1","propagator2","propagator3","propagator4","propagator5","propagator6"]},
        "oem_power":        {"command":"GPS:TEL? 3,data","length":16, "parsing":"<ffff",
            "names":["oem_power0","oem_power1","oem_power2","oem_power3"]}
    },
    "aim2":{
        "status":           {"command":"AIM:TEL? 0,data", "length":2,  "parsing":"hex"},
        "nmea_string":      {"command":"AIM:TEL? 1,data", "length":512,"parsing":"str"},
        "propagator":       {"command":"AIM:TEL? 2,data", "length":56, "parsing":"<ddddddd",
            "names":["propagator0","propagator1","propagator2","propagator3","propagator4","propagator5","propagator6"]},
        "oem_power":        {"command":"AIM:TEL? 3,data", "length":16, "parsing":"<ffff",
            "names":["oem_power0","oem_power1","oem_power2","oem_power3"]},
        "gps_uart":         {"command":"AIM:TEL? 4,data", "length":1,  "parsing":"<B"},
        "gps_event_pin":    {"command":"AIM:TEL? 5,data", "length":1,  "parsing":"hex"},
        "gps_power":        {"command":"AIM:TEL? 6,data", "length":1,  "parsing":"hex"},
        "adcs_uart":        {"command":"AIM:TEL? 7,data", "length":1,  "parsing":"hex"},
        "ftdi_chip":        {"command":"AIM:TEL? 8,data", "length":1,  "parsing":"hex"},
        "adcs_power":       {"command":"AIM:TEL? 9,data", "length":1,  "parsing":"hex"},
        "wdt_period":       {"command":"AIM:TEL? 10,data","length":4,  "parsing":"<I"}
    },
    "bim":{
        "temperature":         {"command":"BIM:TEL? 0,data", "length":24,"parsing":"<ffff",
            "names": ["temp0","temp1","temp2","temp3"]},
        "uart_status":         {"command":"BIM:TEL? 1,data", "length":6, "parsing":"<hhh",
            "names": ["uart1_status","uart2_status","uart3_status"]},
        "imu":                 {"command":"BIM:TEL? 2,data", "length":12,"parsing":"<hhhhhh",
            "names": ["imu0","imu1","imu2","imu3","imu4","imu5"]},
        "tini_status":         {"command":"BIM:TEL? 3,data", "length":1, "parsing":"<B"},
        "temp_scaling_offsets":{"command":"BIM:TEL? 4,data", "length":24,"parsing":"<ffffff",
            "names": ["temp0_offset","temp1_offset","temp2_offset","temp3_offset","temp4_offset","temp5_offset"]},
        "temp_scaling_factors":{"command":"BIM:TEL? 5,data", "length":24,"parsing":"<ffffff",
            "names": ["temp0_factor","temp1_factor","temp2_factor","temp3_factor","temp4_factor","temp5_factor"]}
    },
    "pim":{
        "channel_currents": {"command":"PIM:TEL? 0,data", "length":8, "parsing":"<HHHH",
            "names":["channel0_current","channel1_current","channel2_current","channel3_current"]},
        "channel_resistors":{"command":"PIM:TEL? 1,data", "length":8, "parsing":"<HHHH",
            "names":["channel0_resistor","channel1_resistor","channel2_resistor","channel3_resistor"]},
        "channel_limits":   {"command":"PIM:TEL? 2,data", "length":8, "parsing":"<HHHH",
            "names":["channel0_limit","channel1_limit","channel2_limit","channel3_limit"]},
        "channel_offsets":  {"command":"PIM:TEL? 3,data", "length":16,"parsing":"<ffff",
            "names":["channel0_offset","channel1_offset","channel2_offset","channel3_offset"]},
        "channel_factors":  {"command":"PIM:TEL? 4,data", "length":16,"parsing":"<ffff",
            "names":["channel0_factor","channel1_factor","channel2_factor","channel3_factor"]},
        "status":           {"command":"PIM:TEL? 5,data", "length":6, "parsing":"hex"},
        "overcurrent_log":  {"command":"PIM:TEL? 6,data", "length":8, "parsing":"<HHHH",
            "names":["channel0_overcurrent","channel1_overcurrent","channel2_overcurrent","channel3_overcurrent"]},
        "channel_volts":    {"command":"PIM:TEL? 7,data", "length":8, "parsing":"<HHHH",
            "names":["channel0_voltage","channel1_voltage","channel2_voltage","channel3_voltage"]}
    },
    "rhm":{
        "lithium_power":     {"command":"RHM:TEL? 0,data", "length":1,"parsing":"str"},
        "lithium_uart":      {"command":"RHM:TEL? 1,data", "length":1,"parsing":"<B"},
        "lithium_config":    {"command":"RHM:TEL? 2,data", "length":1,"parsing":"hex"},
        "globalstar_power":  {"command":"RHM:TEL? 3,data", "length":1,"parsing":"hex"},
        "globalstar_uart":   {"command":"RHM:TEL? 4,data", "length":1,"parsing":"<B"},
        "globalstar_din_out":{"command":"RHM:TEL? 5,data", "length":1,"parsing":"hex"},
        "globalstar_busy":   {"command":"RHM:TEL? 6,data", "length":1,"parsing":"hex"},
        "watchdog_period":   {"command":"RHM:TEL? 7,data", "length":4,"parsing":"<I"}
    },
    "bsm":{
        "channel_currents":    {"command":"BSM:TEL? 0,data", "length":10,"parsing":"<HHHHH",
            "names":["channel0_current","channel1_current","channel2_current","channel3_current","channel4_current"]},
        "channel_resistors":   {"command":"BSM:TEL? 1,data", "length":10,"parsing":"<HHHHH",
            "names":["channel0_resistor","channel1_resistor","channel2_resistor","channel3_resistor","channel4_resistor"]},
        "channel_limits":      {"command":"BSM:TEL? 2,data", "length":10,"parsing":"<HHHHH",
            "names":["channel0_limit","channel1_limit","channel2_limit","channel3_limit","channel4_limit"]},
        "channel_offsets":     {"command":"BSM:TEL? 3,data", "length":20,"parsing":"<fffff",
            "names":["channel0_offset","channel1_offset","channel2_offset","channel3_offset","channel4_offset"]},
        "channel_factors":     {"command":"BSM:TEL? 4,data", "length":20,"parsing":"<fffff",
            "names":["channel0_factor","channel1_factor","channel2_factor","channel3_factor","channel4_factor"]},
        "status":              {"command":"BSM:TEL? 5,data", "length":1, "parsing":"hex"},
        "overcurrent_log":     {"command":"BSM:TEL? 6,data", "length":10,"parsing":"<HHHHH",
            "names":["channel0_overcurrent","channel1_overcurrent","channel2_overcurrent","channel3_overcurrent","channel4_overcurrent"]},
        "temperature9":        {"command":"BSM:TEL? 7,data",  "length":2, "parsing":"<H"},
        "temperature10":       {"command":"BSM:TEL? 8,data",  "length":2, "parsing":"<H"},
        "temperature11":       {"command":"BSM:TEL? 9,data",  "length":2, "parsing":"<H"},
        "temperature12":       {"command":"BSM:TEL? 10,data", "length":2, "parsing":"<H"},
        "temp_scaling_offsets":{"command":"BSM:TEL? 11,data", "length":16,"parsing":"<ffff",
            "names":["temp9_offset","temp10_offset","temp11_offset","temp12_offset"]},
        "temp_scaling_factors":{"command":"BSM:TEL? 12,data", "length":16,"parsing":"<ffff",
            "names":["temp9_factor","temp10_factor","temp11_factor","temp12_factor"]}
    },
    "bm2":{
        "temperature":         {"command":"BM2:TEL? 8,data",   "length":2, "parsing":"<H"},
        "voltage":             {"command":"BM2:TEL? 9,data",   "length":2, "parsing":"<H"},
        "current":             {"command":"BM2:TEL? 10,data",  "length":2, "parsing":"<h"},
        "avg_current":         {"command":"BM2:TEL? 11,data",  "length":2, "parsing":"<h"},
        "relative_soc":        {"command":"BM2:TEL? 13,data",  "length":1, "parsing":"<B"},
        "absolute_soc":        {"command":"BM2:TEL? 14,data",  "length":1, "parsing":"<B"},
        "remaining_capacity":  {"command":"BM2:TEL? 15,data",  "length":2, "parsing":"<H"},
        "full_capacity":       {"command":"BM2:TEL? 16,data",  "length":2, "parsing":"<H"},
        "time_to_empty":       {"command":"BM2:TEL? 17,data",  "length":2, "parsing":"<H"},
        "avg_to_empty":        {"command":"BM2:TEL? 18,data",  "length":2, "parsing":"<H"},
        "avg_to_full":         {"command":"BM2:TEL? 19,data",  "length":2, "parsing":"<H"},
        "charging_current":    {"command":"BM2:TEL? 20,data",  "length":2, "parsing":"<H"},
        "charging_voltage":    {"command":"BM2:TEL? 21,data",  "length":2, "parsing":"<H"},
        "battery_status":      {"command":"BM2:TEL? 22,data",  "length":2, "parsing":"hex"},
        "cycle_count":         {"command":"BM2:TEL? 23,data",  "length":2, "parsing":"<H"},
        "design_capacity":     {"command":"BM2:TEL? 24,data",  "length":2, "parsing":"<H"},
        "design_voltage":      {"command":"BM2:TEL? 25,data",  "length":2, "parsing":"<H"},
        "temperature1":        {"command":"BM2:TEL? 48,data",  "length":2, "parsing":"<H"},
        "temperature2":        {"command":"BM2:TEL? 49,data",  "length":2, "parsing":"<H"},
        "temperature3":        {"command":"BM2:TEL? 50,data",  "length":2, "parsing":"<H"},
        "temperature4":        {"command":"BM2:TEL? 51,data",  "length":2, "parsing":"<H"},
        "bm2_status":          {"command":"BM2:TEL? 52,data",  "length":1, "parsing":"hex"},
        "perm_fail_time":      {"command":"BM2:TEL? 53,data",  "length":15,"parsing":"str"},
        "perm_fail_register":  {"command":"BM2:TEL? 54,data",  "length":4, "parsing":"hex"},
        "sbs_read":            {"command":"BM2:TEL? 55,data",  "length":32,"parsing":"hex"},
        "flash_read":          {"command":"BM2:TEL? 56,data",  "length":32,"parsing":"hex"},
        "manu_access_read":    {"command":"BM2:TEL? 57,data",  "length":2, "parsing":"hex"},
        "func_return":         {"command":"BM2:TEL? 58,data",  "length":8, "parsing":"hex"},
        "cell4_voltage":       {"command":"BM2:TEL? 60,data",  "length":2, "parsing":"<H"},
        "cell3_voltage":       {"command":"BM2:TEL? 61,data",  "length":2, "parsing":"<H"},
        "cell2_voltage":       {"command":"BM2:TEL? 62,data",  "length":2, "parsing":"<H"},
        "cell1_voltage":       {"command":"BM2:TEL? 63,data",  "length":2, "parsing":"<H"},
        "temperature5":        {"command":"BM2:TEL? 71,data",  "length":2, "parsing":"<H"},
        "temperature6":        {"command":"BM2:TEL? 72,data",  "length":2, "parsing":"<H"},
        "temperature7":        {"command":"BM2:TEL? 73,data",  "length":2, "parsing":"<H"},
        "temperature8":        {"command":"BM2:TEL? 74,data",  "length":2, "parsing":"<H"},
        "temp_scaling_offsets":{"command":"BM2:TEL? 75,data",  "length":24,"parsing":"<ffffff",
            "names":["temp1_offset","temp2_offset","temp3_offset","temp4_offset","temp5_offset","temp6_offset"]},
        "temp_scaling_factors":{"command":"BM2:TEL? 76,data",  "length":24,"parsing":"<ffffff",
            "names":["temp1_factor","temp2_factor","temp3_factor","temp4_factor","temp5_factor","temp6_factor"]},
        "safety_alert":        {"command":"BM2:TEL? 80,data",  "length":2, "parsing":"<H"},
        "safety_status":       {"command":"BM2:TEL? 81,data",  "length":2, "parsing":"<H"},
        "perm_fail_alert":     {"command":"BM2:TEL? 82,data",  "length":2, "parsing":"<H"},
        "perm_fail_status":    {"command":"BM2:TEL? 83,data",  "length":2, "parsing":"<H"},
        "operation_status":    {"command":"BM2:TEL? 84,data",  "length":2, "parsing":"<H"},
        "charging_status":     {"command":"BM2:TEL? 85,data",  "length":2, "parsing":"<H"},
        "pack_voltage":        {"command":"BM2:TEL? 90,data",  "length":2, "parsing":"<H"},
        "avg_voltage":         {"command":"BM2:TEL? 93,data",  "length":2, "parsing":"<H"},
        "ts1_temp":            {"command":"BM2:TEL? 94,data",  "length":2, "parsing":"<h"},
        "ts2_temp":            {"command":"BM2:TEL? 95,data",  "length":2, "parsing":"<h"},
        "safety_alert2":       {"command":"BM2:TEL? 104,data", "length":2, "parsing":"<H"},
        "safety_status2":      {"command":"BM2:TEL? 105,data", "length":2, "parsing":"<H"},
        "perm_fail_alert2":    {"command":"BM2:TEL? 106,data", "length":2, "parsing":"<H"},
        "perm_fail_status2":   {"command":"BM2:TEL? 107,data", "length":2, "parsing":"<H"},
        "temp_range":          {"command":"BM2:TEL? 114,data", "length":2, "parsing":"<H"}
    }
}
# End Config Data
#################

class MCU:
    
    def __init__(self,address):
        """
        Sets the bus number, stores the address and configuration data
        """
        self.i2cfile = i2c.I2C(bus = I2C_BUS_NUM)
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
            read_data = self._header_parse(input_dict['length']+HEADER_SIZE)
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
            # Search for the null terminator, return the leading string it in a tuple
            str_data = data.split('\0')[0]
            if str_data == '':
                str_data = "No valid data received, raw data:"+data
            return (str_data,)
        elif parsing == "hex":
            # Store as a hex string. This is so we can return binary data.
            # Return as a single field in a tuple
            return (data.encode('hex'),)
        
        # All others parse directly with the parsing string. 
        return struct.unpack(parsing,data)