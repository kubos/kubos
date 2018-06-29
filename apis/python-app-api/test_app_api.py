#!/usr/bin/env python

# Copyright 2018 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.

"""
Unit testing for the I2C library.
"""

import app_api
import unittest
# import mock
# from contextlib import nested


class TestAppAPI(unittest.TestCase):

    def setUp(self):
        self.api = app_api.Services(
            config_filepath="/Users/jessecoffey/Workspace/apollo-fusion" +
            "/common/overlay/home/system/etc/config.toml")

    def test_query_servicetype(self):
        bad_service = 1
        query = "query {moduleList}"
        with self.assertRaises(KeyError):
            self.api.query(service=bad_service, query=query)

    def test_query_querytype(self):
        service = "app-service"
        bad_query = 20
        with self.assertRaises(TypeError):
            self.api.query(service=service, query=bad_query)


if __name__ == '__main__':
    unittest.main()
