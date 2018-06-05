#!/usr/bin/env python

# Copyright 2018 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.

"""
API for interacting with all Pumpkin SupMCUs. 

Unit test module for the pumpkin mcu api
"""

import unittest
import mcu_api
import mock

class TestMCUAPI(unittest.TestCase):

	def setUp(self):
		self.mcu = mcu_api.MCU(address = 0x20)

	def test_command_type(self):
		with self.assertRaises(TypeError):
			bad_command = 23 # Not a string
			self.mcu.write(command = bad_command)

	def test_stopbyte_appending(self):
		fake_command = "SUP:LED ON"
		with mock.patch('i2c.I2C.write') as mock_i2cwrite:
			self.mcu.write(command = fake_command)
			mock_i2cwrite.assert_called_with(
				data = fake_command + '\x0a',
				device = self.mcu.address)

	def test_read(self):
		read_count = 20
		with mock.patch('i2c.I2C.read') as mock_i2cread:
			self.mcu.read(count = read_count)
			mock_i2cread.assert_called_with(
				device = self.mcu.address,
				count = read_count)

	

if __name__ == '__main__':
    unittest.main()