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
import mcu_api

# Get what modules are present 
# TO DO: pass this in from service config file
MODULES = {
    "sim":  {"address":80},
    "gpsrm":{"address":81},
    "aim2": {"address":0},
    "bim":  {"address":0},
    "pim":  {"address":83},
    "rhm":  {"address":85},
    "bsm":  {"address":0},
    "bm2":  {"address":92}
 }

class Query(graphene.ObjectType):
    """
    Creates query endpoints exposed by graphene.
    """
    moduleList = graphene.JSONString()
    fieldList = graphene.List(graphene.String,module=graphene.String())
    read = graphene.String(
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
        telemetry = mcu_api.TELEMETRY
        fields = []
        for field in telemetry["supervisor"]:
            fields.append(field)
        for field in telemetry[module]:
            fields.append(field)
        return fields
    
    def resolve_read(self, info, module, count):
        """
        Reads number of bytes from the specified MCU
        """
        if module not in MODULES:
            raise KeyError('Module not configured',module)
        address = MODULES[module]['address']
        if address == 0:
            raise ValueError('Module not present',module)
        mcu = mcu_api.MCU(address = address)
        bin_data = mcu.read(count = count)
        
        return bin_data.encode("hex")
    
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
        mcu = mcu_api.MCU(address = address,config_data = API_CONFIG_DATA)
        out = mcu.read_telemetry(module = module,fields = fields)
        return json.dumps(out)

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
        mcu = mcu_api.MCU(address = address,config_data = API_CONFIG_DATA)
        out = mcu.write(command)
        
        commandStatus = CommandStatus(status = out[0], command = out[1])
        
        return commandStatus


class Mutation(graphene.ObjectType):
    """
    Creates mutation endpoints exposed by graphene.
    """

    passthrough = Passthrough.Field()

schema = graphene.Schema(query=Query, mutation=Mutation)