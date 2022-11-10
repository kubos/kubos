#!/usr/bin/env python3

# Copyright 2018 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.

"""
Unit testing for the Kubos App API.
"""

import kubos_app
import unittest
import mock
import responses
import toml

from requests.exceptions import ConnectionError, HTTPError

class TestAppAPI(unittest.TestCase):

    def setUp(self):
        self.api = kubos_app.Services("test_config.toml")

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

    @responses.activate
    def test_query(self):
        service = "test-service"
        query = "test query"
        ip = "0.0.0.0"
        port = 8000
        
        responses.add(
            responses.POST, 'http://0.0.0.0:8000',
            json={'data':'test data'},
            status=200)
        
        response = self.api.query(service=service, query=query)
        
        assert response == "test data"

    @responses.activate
    def test_timeout(self):
        service = "test-service"
        query = "test query"
        timeout = 0.03
        
        responses.add(
            responses.POST, 'http://0.0.0.0:8000',
            json={'data':'test data'},
            status=200)
        
        response = self.api.query(service=service, query=query, timeout=timeout)
        assert response == "test data"

    @responses.activate
    def test_endpoint_error(self):
        service = "test-service"
        query = "test query"
        timeout = 0.03
        
        responses.add(
            responses.POST, 'http://0.0.0.0:8000',
            json={'errors':["Lots of errors"],'data':""},
            status=200)
        
        with self.assertRaises(EnvironmentError):
            self.api.query(service=service, query=query, timeout=timeout)

    @responses.activate
    def test_errs_field_error(self):
        service = "test-service"
        query = "test query"
        timeout = 0.03
        
        responses.add(
            responses.POST, 'http://0.0.0.0:8000',
            json={'bad_field_name':[], 'data':""},
            status=200)
        
        with self.assertRaises(KeyError):
            self.api.query(service=service, query=query, timeout=timeout)

    @responses.activate
    def test_msg_field_error(self):
        service = "test-service"
        query = "test query"
        timeout = 0.03
        
        responses.add(
            responses.POST, 'http://0.0.0.0:8000',
            json={'bad_field_name':[], 'errors':""},
            status=200)
        
        with self.assertRaises(KeyError):
            self.api.query(service=service, query=query, timeout=timeout)
            
    @responses.activate
    def test_bad_url(self):
        service = "test-service"
        query = "test query"
        ip = "0.0.0.0"
        port = 8000
        
        responses.add(
            responses.POST, 'http://0.0.0.0:8001',
            json={'data':'test data'},
            status=200)
        
        with self.assertRaises(ConnectionError):
            response = self.api.query(service=service, query=query)
            
    @responses.activate
    def test_bad_status(self):
        service = "test-service"
        query = "test query"
        ip = "0.0.0.0"
        port = 8000
        
        responses.add(
            responses.POST, 'http://0.0.0.0:8000',
            status=404)
        
        with self.assertRaises(HTTPError):
            response = self.api.query(service=service, query=query)

if __name__ == '__main__':
    unittest.main()
