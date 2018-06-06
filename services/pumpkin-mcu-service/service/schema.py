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

# Initialize MODULES global. This is then configured in the service file. 
# "module_name" must match an entry in the API configuration dict
# "address" must be a valid I2C address int.
MODULES = {
    "module_name": {"address":0xFF} 
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
        """
        This allows discovery of which modules are present and what 
        addresses they have. Mostly just a debugging/discovery tool.
        """
        return json.dumps(MODULES)
        
    def resolve_fieldList(self, info, module):
        """
        This allows discovery of which fields are available for a 
        specific module. Mostly just a debugging/discovery tool. 
        """
        if module not in MODULES:
            raise KeyError('Module not configured',module)
        address = MODULES[module]['address']
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
        Returns as a hex string. 
        """
        if module not in MODULES:
            raise KeyError('Module not configured',module)
        address = MODULES[module]['address']
        mcu = mcu_api.MCU(address = address)
        bin_data = mcu.read(count = count)
        
        return bin_data.encode("hex")
    
    def resolve_mcuTelemetry(self, info, module, fields):
        """
        Queries specific telemetry item fields from the speficied
        module. 
        
        fields must be a list of value field names matching the 
        configuration data in the mcu_api.py file. Inputting ['all']
        retrieves all available telemetry for that module. 

        Retuns json dump of the form:
        {
        'fieldname1':{'timestamp':float,'data':configured datatype},
        'fieldname2':{'timestamp':float,'data':configured datatype}
        }
        """
        if module not in MODULES:
            raise KeyError('Module not configured',module)
        address = MODULES[module]['address']
        fields = map(str,fields)
        mcu = mcu_api.MCU(address = address)
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