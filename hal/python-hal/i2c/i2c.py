#!/usr/bin/env python

# Copyright 2017 Kubos Corporation
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
        self.readfile = io.open("/dev/i2c-"+str(bus), "rb", buffering=0)
        self.writefile = io.open("/dev/i2c-"+str(bus), "wb", buffering=0)
        
        fcntl.ioctl(self.readfile, I2C_SLAVE, device)
        fcntl.ioctl(self.writefile, I2C_SLAVE, device)
    
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
        self.writefile.write(data)
        return True,data
    
    def read(self, count):
        """
        Reads the specified number of bytes from the device. 
        """
        return self.readfile.read(count)
    
    def close(self):
        """
        Closes the files.
        """
        self.writefile.close()
        self.readfile.close()
        

    