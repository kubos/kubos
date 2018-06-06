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

print "Service IP:", c.ip
print "Service port:", c.port
print "Testing port:", testing_port

sock = socket.socket(socket.AF_INET,  # Internet
                     socket.SOCK_DGRAM)  # UDP
sock.bind((c.ip, testing_port))

print "\n###########################"
print "Query Modules and Addresses"
query = 'query {moduleList}'
print "\nquery:", query
sock.sendto(query, (c.ip, c.port))
raw_modules, addr = sock.recvfrom(1024)
print raw_modules

# turn modules into a proper dictionary
raw_modules2 = json.loads(raw_modules)['msg']['moduleList']
modules = json.loads(raw_modules2)

print "\n#################"
print "fieldList queries"
for module in modules:
    query = 'query {fieldList(module: "'+str(module)+'")}'
    print "\nquery:", query
    sock.sendto(query, (c.ip, c.port))
    fieldList, addr = sock.recvfrom(1024)
    print 'fieldList: ' + str(fieldList)


print "\n####################"
print "mcuTelemetry queries"
for module in modules:
    query = 'query {mcuTelemetry(module: "'+str(module)+'")}'
    print "\nquery:", query
    sock.sendto(query, (c.ip, c.port))
    mcuTelemetry, addr = sock.recvfrom(1024)
    print 'All fields for mcuTelemetry: '+str(mcuTelemetry)
