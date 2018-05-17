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
    
    rawRead = graphene.String(
        module=graphene.String(),
        count=graphene.Int())
        
    # def resolve_mcuTelemetry(self, info, module, fields):
    #     """
    #     Handles request for subsystem query.
    #     """
    #     if module not in MODULES:
    #         raise KeyError('Module not configured',module)
    #     address = MODULES[module]['address']
    #     if address == 0:
    #         raise ValueError('Module not present',module)
    #     fields = map(str,fields)
    #     mcu = mcu_api.MCU(address = address)
    #     out = mcu.get_module_telemetry(module = module,fields = fields)
    #     # out = {"field1":{"timestamp": 152349502.1,"data":12345},
    #     #     "field2": {"timestamp": 152349502.1,"data":"stuffandthings"},
    #     #     "field3": {"timestamp": 152349502.1,"data":1245234.212415},
    #     #     "field4": {"timestamp": 152349502.1,"data":"\x12\x23\x34\x45\x56\x67\x78\x89\x0a"}}
    #     return out
    
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
        out = mcu.
        return 
        


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