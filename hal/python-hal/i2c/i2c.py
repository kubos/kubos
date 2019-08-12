#!/usr/bin/env python3

# Copyright 2018 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.
# Referenced from: https://www.raspberrypi.org/forums/viewtopic.php?t=162248&p=1049717

"""
I2C Library
"""

import io
import sys
import fcntl

I2C_SLAVE = 0x0703


class I2C:

    def __init__(self, bus):
        """
        Retrieves the read/write file handle for the device
        """
        self.filepath = "/dev/i2c-"+str(bus)

    def write(self, device, data):
        """
        Sets the address for the device.
        Formats the data and writes the data to the device.
        Input must be a string or a list.
        Returns True and the data (as written to the device) if successful
        """

        with io.open(self.filepath, "r+b", buffering=0) as file:

            fcntl.ioctl(file, I2C_SLAVE, device)

            if type(data) is list:
                data = bytearray(data)
            elif type(data) is bytes:
                pass
            else:
                raise TypeError('Invalid data format: ' +
                                str(type(data))+', must be bytes or list')
            file.write(data)
            return True, data

    def read(self, device, count):
        """
        Reads the specified number of bytes from the device.
        """
        with io.open(self.filepath, "r+b", buffering=0) as file:
            fcntl.ioctl(file, I2C_SLAVE, device)

            return file.read(count)
