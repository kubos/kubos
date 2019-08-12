#!/usr/bin/env python3

# Copyright 2018 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.


"""
Wrapper for creating a UDP based Kubos service
"""

import socket
import json
import logging


def start(logger, config, schema, context={}):
    logger.info("{} starting on {}:{}".format(config.name, config.ip, config.port))
    sock = socket.socket(socket.AF_INET,  # Internet
                         socket.SOCK_DGRAM)  # UDP
    sock.bind((config.ip, config.port))
    base_schema = schema.schema

    while True:
        try:
            data, source = sock.recvfrom(1024)
            errs = None
            msg = None
            try:
                result = base_schema.execute(data.decode(), context_value=context)
                msg = result.data
                if result.errors:
                    errs = []
                    for e in result.errors:
                        errs.append(str(e))

            except Exception as e:
                errs = "Exception encountered {}".format(e)

            result = json.dumps({
                "data": msg,
                "errors": errs
            })
            sock.sendto(str.encode(result), source)
        except Exception as e:
            logging.error("Exception encountered {}".format(e))
