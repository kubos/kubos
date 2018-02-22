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
        self.i2cdevice = i2c.I2C(1, "/dev/123")

    def test_datatype(self):
        with nested(
                mock.patch('io.open'),
                mock.patch('fcntl.ioctl'),
        ) as (mock_open, mock_ioctl):
            with self.assertRaises(TypeError):
                self.i2cdevice.write(1, 123)


if __name__ == '__main__':
    unittest.main()
