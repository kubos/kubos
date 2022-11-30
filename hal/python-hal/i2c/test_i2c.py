#!/usr/bin/env python3

# Copyright 2018 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.

"""
Unit testing for the I2C library.
"""

import unittest
import i2c
from unittest import mock

class TestI2C(unittest.TestCase):

    def setUp(self):
        self.i2cdevice = i2c.I2C(1)

    def test_filepath(self):
        self.assertEqual("/dev/i2c-1", self.i2cdevice.filepath)

    def test_handles_list_data(self):
        with mock.patch('io.open'), mock.patch('fcntl.ioctl'):
            assert self.i2cdevice.write(1, [0x73])

    def test_sets_i2c_as_slave(self):
        fake_device = 1
        fake_data = b'fake'

        with mock.patch('io.open'), mock.patch('fcntl.ioctl') as mock_ioctl:
            self.i2cdevice.write(fake_device, fake_data)
            mock_ioctl.assert_called_with(mock.ANY, i2c.I2C_SLAVE, fake_device)

    def test_wrong_datatype_raises_type_error(self):
        bad_datatype = 123  # Not a string or list
        with mock.patch('io.open'), mock.patch('fcntl.ioctl'):
            with self.assertRaises(TypeError):
                self.i2cdevice.write(1, bad_datatype)

    def test_write(self):
        fake_device = 1
        fake_data = b'fake'

        with mock.patch('io.open'), mock.patch('fcntl.ioctl') as mock_ioctl:
            self.assertTrue(self.i2cdevice.write(fake_device, fake_data)[0])

    def test_read_sets_i2c_as_slave(self):
        fake_device = 1
        fake_count = 30

        with mock.patch('io.open'), mock.patch('fcntl.ioctl') as mock_ioctl:
            self.i2cdevice.read(fake_device, fake_count)
            mock_ioctl.assert_called_with(mock.ANY, i2c.I2C_SLAVE, fake_device)


if __name__ == '__main__':
    unittest.main()
