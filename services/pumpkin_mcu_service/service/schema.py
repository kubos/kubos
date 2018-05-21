#!/usr/bin/env python

# Copyright 2018 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.

"""
Graphene schema setup to enable queries.
"""

import graphene
from models import *
import json

from pumpkin_mcu_api import mcu_api

# Get what modules are present from config file
MODULES = mcu_api.CONFIG_DATA['modules']

class Query(graphene.ObjectType):
    """
    Creates query endpoints exposed by graphene.
    """
    moduleList = graphene.JSONString()
    fieldList = graphene.List(module=graphene.String())
    rawRead = graphene.String(
        module=graphene.String(),
        count=graphene.Int())
    mcuTelemetry = graphene.JSONString(
        module=graphene.String(),
        fields=graphene.List(graphene.String,default_value = ["all"]))
    
    def resolve_moduleList(self, info):
        return json.dumps(MODULES)
        
    def resolve_fieldList(self, info, module):
        if module not in MODULES:
            raise KeyError('Module not configured',module)
        address = MODULES[module]['address']
        if address == 0:
            raise ValueError('Module not present',module)
        telemetry = mcu_api.CONFIG_DATA['telemetry']
        fields = []
        for field in telemetry["supervisor"]:
            fields.append(field)
        for field in telemetry[module]:
            fields.append(field)
        return fields
    
    def resolve_mcuTelemetry(self, info, module, fields):
        """
        Handles request for subsystem query.
        """
        if module not in MODULES:
            raise KeyError('Module not configured',module)
        address = MODULES[module]['address']
        if address == 0:
            raise ValueError('Module not present',module)
        fields = map(str,fields)
        mcu = mcu_api.MCU(address = address)
        out = mcu.get_module_telemetry(module = module,fields = fields)
        return json.dumps(out)
    
    def resolve_rawRead(self, info, module, count):
        """
        Reads number of bytes from the specified MCU
        """
        if module not in MODULES:
            raise KeyError('Module not configured',module)
        address = MODULES[module]['address']
        if address == 0:
            raise ValueError('Module not present',module)
        mcu = mcu_api.MCU(address = address)
        bin_data = mcu.raw_read(count = count)
        
        return bin_data.encode("hex")

class Passthrough(graphene.Mutation):
    """
    Creates mutation for Passthrough Module Commanding
    """

    class Arguments:
        module = graphene.String()
        command = graphene.String()
        
    Output = CommandStatus

    def mutate(self, info, module, command):
        """
        Handles passthrough commands to the Pumpkin MCU modules. 
        """
        if module not in MODULES:
            raise KeyError('Module not configured',module)
        address = MODULES[module]['address']
        if address == 0:
            raise KeyError('Module not present',module)
        if type(command) == unicode: command = str(command)
        mcu = mcu_api.MCU(address = address)
        out = mcu.write(command)
        
        commandStatus = CommandStatus(status = out[0], command = out[1])
        
        return commandStatus


class Mutation(graphene.ObjectType):
    """
    Creates mutation endpoints exposed by graphene.
    """

    passthrough = Passthrough.Field()

schema = graphene.Schema(query=Query, mutation=Mutation)