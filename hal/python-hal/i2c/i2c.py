#!/usr/bin/env python

# Copyright 2018 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.
# Referenced from: https://www.raspberrypi.org/forums/viewtopic.php?t=162248&p=1049717

"""
I2C Library
"""

import io,sys,fcntl

I2C_SLAVE=0x0703

class I2C:

    def __init__(self, device, bus):
        """
        Retrieves the read/write file handles for the device and sets the device address
        """
        
        self.devfile = io.open("/dev/i2c-"+str(bus), "r+b", buffering=0)
        
        fcntl.ioctl(self.devfile, I2C_SLAVE, device)
    
    def write(self, data):
        """
        Formats the data and writes the data to the device. 
        Input must be a string or a list.
        Returns True and the data (as written to the device) if successful
        """
        if type(data) is list:
            data = bytearray(data)
        elif type(data) is str:
            pass
        else: 
            raise('Invalid data format: '+str(type(data))+', must be string or list')
        self.devfile.write(data)
        return True,data
    
    def read(self, count):
        """
        Reads the specified number of bytes from the device. 
        """
        return self.devfile.read(count)
    
    def close(self):
        """
        Closes the files.
        """
        self.devfile.close()
        

    