#!/usr/bin/env python

# Copyright 2018 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.

"""
Integration test for the Pumpkin MCU Service. 

Runs the moduleList query
"""

import socket
from kubos_service.config import Config
import json


print "\n###################################"
print "Service and Test Port Configuration"
c = Config("pumpkin-mcu-service")
testing_port = c.port + 1000
ERRORS = {}

print "Service IP:", c.ip
print "Service port:", c.port
print "Testing port:", testing_port

sock = socket.socket(socket.AF_INET,  # Internet
                     socket.SOCK_DGRAM)  # UDP
sock.bind((c.ip, testing_port))

print "\n###########################"
print "Query Modules and Addresses"
query = 'query {moduleList}'
print "\nquery: " + query
sock.sendto(query, (c.ip, c.port))
raw_modules, addr = sock.recvfrom(1024)
print raw_modules

# turn modules into a proper dictionary
dict_result = json.loads(raw_modules)
modules = json.loads(dict_result['msg']['moduleList'])

print "\n#################################"
print "fieldList queries for all modules"
for module in modules:
    query = 'query {fieldList(module: "' + module + '")}'
    print "\nquery: " + query
    sock.sendto(query, (c.ip, c.port))
    fieldList, addr = sock.recvfrom(1024)
    print 'fieldList: ' + fieldList
    dict_result_fieldList = json.loads(fieldList)
    if dict_result_fieldList['errs'] != None:
        ERRORS.update({query: dict_result_fieldList['errs']})


print "\n####################################"
print "mcuTelemetry queries for all modules"
for module in modules:
    query = 'query {mcuTelemetry(module: "' + module + '")}'
    print "\nquery:", query
    sock.sendto(query, (c.ip, c.port))
    mcuTelemetry, addr = sock.recvfrom(1024)
    print 'All fields for mcuTelemetry: ' + mcuTelemetry
    dict_result_mcuTelemetry = json.loads(mcuTelemetry)
    if dict_result_mcuTelemetry['errs'] != None:
        ERRORS.update({query: dict_result_mcuTelemetry['errs']})

print "\n############"
print "Errors"
print ERRORS
