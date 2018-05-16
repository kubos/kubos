#!/usr/bin/env python

# Copyright 2017 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.

"""
Graphene schema setup to enable queries.
"""

import graphene
from models import *

from pumpkin_mcu_api import mcu_api

# Get what modules are present from config file
MODULES = mcu_api.CONFIG_DATA['modules']

# Local subsystem instance for tracking state
# May not be neccesary when tied into actual hardware
#_local = Subsystem(power_on=False)


class Query(graphene.ObjectType):
    """
    Creates query endpoints exposed by graphene.
    """
    
    mcuTelemetry = graphene.TelemetryArguments(
        module=graphene.String(),
        field=graphene.String())
    hello = graphene.String(name=graphene.String(default_value="stranger"))

    def resolve_hello(self, info, name):
        return 'Hello ' + name
    def resolve_mcuTelemetry(self, info, module, field):
        """
        Handles request for subsystem query.
        """
        if module not in MODULES:
            raise KeyError('Module not configured',module)
        address = MODULES[module]['address']
        if address == 0:
            raise ValueError('Module not present',module)
        if type(field) == unicode: field = str(field)
        mcu = mcu_api.MCU(address = address)
        out = mcu.get_module_telemetry(module = module,fields = [field])
        
        readData = ReadData(
            timestamp = out[field]['timestamp'], 
            data = out[field]['data'])
        
        return readData


class CommandPassthrough(graphene.Mutation):
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

class ReadPassthrough(graphene.Mutation):
    """
    Creates mutation for Passthrough Module Commanding
    """

    class Arguments:
        module = graphene.String()
        count = graphene.Int()
        
    Output = ReadData

    def mutate(self, info, module, count):
        """
        Handles request for subsystem query.
        """
        if module not in MODULES:
            raise KeyError('Module not configured',module)
        address = MODULES[module]['address']
        if address == 0:
            raise KeyError('Module not present',module)
        mcu = mcu_api.MCU(address = address)
        out = mcu.read(count = count)
        
        readData = ReadData(
            timestamp = out['timestamp'], 
            data = out['data'])
        
        return readData


class Mutation(graphene.ObjectType):
    """
    Creates mutation endpoints exposed by graphene.
    """

    commandPassthrough = CommandPassthrough.Field()
    readPassthrough = ReadPassthrough.Field()

schema = graphene.Schema(query=Query, mutation=Mutation)
