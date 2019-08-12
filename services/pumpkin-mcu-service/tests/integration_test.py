#!/usr/bin/env python3

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


print("\n###################################")
print("Service and Test Port Configuration")
c = Config("pumpkin-mcu-service")
testing_port = c.port + 1000
ERRORS = {}

print("Service IP:", c.ip)
print("Service port:", c.port)
print("Testing port:", testing_port)

sock = socket.socket(socket.AF_INET,  # Internet
                     socket.SOCK_DGRAM)  # UDP
sock.bind((c.ip, testing_port))

print("\n###########################")
print("Query Modules and Addresses")
query = b'query {moduleList}'
print("\nquery: {}".format(query))
sock.sendto(query, (c.ip, c.port))
raw_modules, addr = sock.recvfrom(1024)
print(raw_modules)

# turn modules into a proper dictionary
dict_result = json.loads(raw_modules.decode())
modules = json.loads(dict_result['data']['moduleList'])

print("\n#################################")
print("fieldList queries for all modules")
for module in modules:
    query = b'query {fieldList(module: "' + str.encode(module) + b'")}'
    print("\nquery: {}".format(query))
    sock.sendto(query, (c.ip, c.port))
    fieldList, addr = sock.recvfrom(1024)
    print('fieldList: {}'.format(fieldList))
    dict_result_fieldList = json.loads(fieldList.decode())
    if dict_result_fieldList['errors'] != None:
        ERRORS.update({query: dict_result_fieldList['errors']})


print("\n####################################")
print("mcuTelemetry queries for all modules")
for module in modules:
    query = b'query {mcuTelemetry(module: "' + str.encode(module) + b'")}'
    print("\nquery: {}".format(query))
    sock.sendto(query, (c.ip, c.port))
    mcuTelemetry, addr = sock.recvfrom(4096)
    print('All fields for mcuTelemetry: {}'.format(mcuTelemetry))
    dict_result_mcuTelemetry = json.loads(mcuTelemetry.decode())
    if dict_result_mcuTelemetry['errors'] != None:
        ERRORS.update({query: dict_result_mcuTelemetry['errors']})

print("\n############")
print("Errors")
print(ERRORS)
