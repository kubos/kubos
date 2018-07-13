#!/usr/bin/env python

# Copyright 2018 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.

"""
Unit testing for the I2C library.
"""

import app_api
import unittest
import mock
import json


class TestAppAPI(unittest.TestCase):

    def setUp(self):
        self.api = app_api.Services("test_config.toml")

    def test_query_servicetype(self):
        bad_service = 1
        query = "query {moduleList}"
        with self.assertRaises(KeyError):
            self.api.query(service=bad_service, query=query)

    def test_query_querytype(self):
        service = "test-service"
        bad_query = 20
        with self.assertRaises(TypeError):
            self.api.query(service=service, query=bad_query)

    def test_query(self):
        service = "test-service"
        query = "test query"
        ip = "0.0.0.0"
        port = 8000
        query_response = '{"errs":[],"msg":"test data"}'
        timeout = 0.03
        with mock.patch('socket.socket') as mock_sock:
            mock_sock.return_value.recv.return_value = query_response
            self.api.query(service=service, query=query, timeout=timeout)
            mock_sock.return_value.sendto.assert_called_with(query, (ip, port))

    def test_timeout(self):
        service = "test-service"
        query = "test query"
        query_response = '{"errs":[],"msg":"test data"}'
        timeout = 0.03
        with mock.patch('socket.socket') as mock_sock:
            mock_sock.return_value.recv.return_value = query_response
            self.api.query(service=service, query=query, timeout=timeout)
            mock_sock.return_value.settimeout.assert_called_with(timeout)

    def test_endpoint_error(self):
        service = "test-service"
        query = "test query"
        bad_query_response = '{"errs":["Lots of errors"],"msg":""}'
        timeout = 0.03
        with mock.patch('socket.socket') as mock_sock:
            mock_sock.return_value.recv.return_value = bad_query_response
            with self.assertRaises(EnvironmentError):
                self.api.query(service=service, query=query, timeout=timeout)

    def test_errs_field_error(self):
        service = "test-service"
        query = "test query"
        bad_query_response = '{"bad_field_name":[], "msg":""}'
        timeout = 0.03
        with mock.patch('socket.socket') as mock_sock:
            mock_sock.return_value.recv.return_value = bad_query_response
            with self.assertRaises(KeyError):
                self.api.query(service=service, query=query, timeout=timeout)

    def test_msg_field_error(self):
        service = "test-service"
        query = "test query"
        bad_query_response = '{"bad_field_name":[], "errs":""}'
        timeout = 0.03
        with mock.patch('socket.socket') as mock_sock:
            mock_sock.return_value.recv.return_value = bad_query_response
            with self.assertRaises(KeyError):
                self.api.query(service=service, query=query, timeout=timeout)


if __name__ == '__main__':
    unittest.main()
