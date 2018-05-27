#!/usr/bin/env python

# Copyright 2018 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.


"""
Wrapper for creating a UDP based Kubos service
"""

import graphene
import socket
import json

def start(config, schema):
    print "{} starting on {}:{}".format(config.name, config.ip, config.port)
    sock = socket.socket(socket.AF_INET, # Internet
                         socket.SOCK_DGRAM) # UDP
    sock.bind((config.ip, config.port))
    base_schema = schema.schema

    while True:
        try:
            data, source = sock.recvfrom(1024)
            errs = None
            msg = None
            try:
                result = base_schema.execute(data)
                msg = result.data
                if result.errors:
                    errs = []
                    for e in result.errors:
                        errs.append(e.message)

            except Exception as e:
                errs = "Exception encountered {}".format(e)

            result = json.dumps({
                "msg" : msg,
                "errs" : errs
            })
            sock.sendto(result, source)
        except Exception as e:
            print "Exception encountered {}".format(e)
