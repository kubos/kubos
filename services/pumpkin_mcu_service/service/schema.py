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
_command_count = 0#Subsystem(power_on=False)


class Query(graphene.ObjectType):
    """
    Creates query endpoints exposed by graphene.
    """

    module = graphene.String()
    count = graphene.Int()

    def read(self, module, count):
        """
        Handles request for subsystem query.
        """
        address = MODULES[module]['address']
        mcu = mcu_api.MCU(address = address)
        out = mcu.read(count = count)
        
        readData = ReadData(status = out[0], command = out[1])
        
        return readData


class Passthrough(graphene.Mutation):
    """
    Creates mutation for Passthrough Module Commanding
    """

    class Arguments:
        module = graphene.String()
        command = graphene.String()
        
    Output = CommandStatus

    def mutate(self, module, command):
        """
        Handles passthrough commands to the Pumpkin MCU modules. 
        """
        # address = MODULES[module]['address']
        # mcu = mcu_api.MCU(address = address)
        # out = mcu.write(command)
        
        commandStatus = CommandStatus(status = True, command = "heythere")
        
        return commandStatus


class Mutation(graphene.ObjectType):
    """
    Creates mutation endpoints exposed by graphene.
    """

    passthrough = Passthrough.Field()

schema = graphene.Schema(query=Query, mutation=Mutation)
