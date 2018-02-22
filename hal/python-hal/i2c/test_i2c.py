#!/usr/bin/env python

# Copyright 2018 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.

"""
Unit testing for the I2C library.
"""

import unittest
import i2c
import mock
from contextlib import nested

class TestI2C(unittest.TestCase):

    def setUp(self):
        self.i2cdevice = i2c.I2C(1)

    def test_handles_list_data(self):
        fake_device = 1
        fake_data = ["s"]

        with nested(
                mock.patch('io.open'),
                mock.patch('fcntl.ioctl'),
        ) as (mock_open, mock_ioctl):
                assert self.i2cdevice.write(fake_device, fake_data)

    def test_sets_i2c_as_slave(self):
        fake_device = 1
        fake_data = "fake"

        with nested(
                mock.patch('io.open'),
                mock.patch('fcntl.ioctl'),
        ) as (mock_open, mock_ioctl):
            self.i2cdevice.write(fake_device, fake_data)
            mock_ioctl.assert_called_with(mock.ANY, i2c.I2C_SLAVE, fake_device)

    def test_datatype(self):
        fake_device = 1
        fake_data = 123

        with nested(
                mock.patch('io.open'),
                mock.patch('fcntl.ioctl'),
        ) as (mock_open, mock_ioctl):
            with self.assertRaises(TypeError):
                self.i2cdevice.write(fake_device, fake_data)


if __name__ == '__main__':
    unittest.main()
