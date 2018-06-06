#!/usr/bin/env python

# Copyright 2018 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.

"""
Integration test for the Pumpkin MCU Service
"""

import socket
from kubos_service.config import Config

c = Config("pumpkin-mcu-service")
testing_port = c.port + 1000

query = 'query {moduleList}'

print "Service IP:", c.ip
print "Service port:", c.port
print "Testing port:", testing_port
print "query:", query

sock = socket.socket(socket.AF_INET, # Internet
             		 socket.SOCK_DGRAM) # UDP
sock.bind((c.ip,testing_port))
sock.sendto(query, (c.ip, c.port))
data,addr = sock.recvfrom(1024)
print data,addr