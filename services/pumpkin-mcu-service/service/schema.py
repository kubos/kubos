#!/usr/bin/env python

# Copyright 2018 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.

"""
Graphene schema setup to enable queries.
"""

import graphene
from models import *
import mcu_api

# Initialize MODULES global. This is then configured in the service file.
# "module_name" must match an entry in the API configuration dict
# "address" must be a valid I2C address int.
MODULES = {
    "module_name": {"address": 0xFF}
}


class Query(graphene.ObjectType):
    """
    Creates query endpoints exposed by graphene.
    """
    moduleList = graphene.JSONString()
    fieldList = graphene.List(graphene.String, module=graphene.String())
    read = graphene.String(
        module=graphene.String(),
        count=graphene.Int())
    mcuTelemetry = graphene.JSONString(
        module=graphene.String(),
        fields=graphene.List(graphene.String, default_value=["all"]))

    def resolve_moduleList(self, info):
        """
        This allows discovery of which modules are present and what
        addresses they have. Mostly just a debugging/discovery tool.
        """
        return MODULES

    def resolve_fieldList(self, info, module):
        """
        This allows discovery of which fields are available for a
        specific module. Mostly just a debugging/discovery tool.
        """
        if module not in MODULES:
            raise KeyError('Module not configured: {}'.format(module))
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
            raise KeyError('Module not configured: {}'.format(module))
        address = MODULES[module]['address']
        mcu = mcu_api.MCU(address=address)
        bin_data = mcu.read(count=count)

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
            raise KeyError('Module not configured: {}'.format(module))
        address = MODULES[module]['address']
        fields = map(str, fields)
        mcu = mcu_api.MCU(address=address)
        out = mcu.read_telemetry(module=module, fields=fields)
        return out


class Ping(graphene.Mutation):
    """
    Service noop mutation
    Confirms the service is running, but not that it's talking to hardware
    """
    Output = TestResults

    def mutate(self, info):
        """
        Pong
        """
        testResults = TestResults(success=True,
                                  errors=[],
                                  results="Pong")
        return testResults


class Noop(graphene.Mutation):
    """
    Hardware noop mutation
    """
    Output = TestResults

    def mutate(self, info):
        """
        Run firmware version telem request on all modules as a noop
        """
        success = True
        errors = []
        test_output = {}
        for module in MODULES:
            try:
                mcu = mcu_api.MCU(address=MODULES[module]['address'])
                out = mcu.read_telemetry(
                    module=module,
                    fields=['firmware_version'])
                mcu_out = {module: out}
                test_output.update(mcu_out)
            except Exception as e:
                success = False
                errors.append(
                    'Error with module : {} : {}'.format(module, e))

        testResults = TestResults(errors=errors,
                                  success=success,
                                  results=test_output)
        return testResults


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
            raise KeyError('Module not configured', module)
        if type(command) == unicode:
            command = str(command)
        mcu = mcu_api.MCU(address=MODULES[module]['address'])
        out = mcu.write(command)

        commandStatus = CommandStatus(status=out[0], command=out[1])

        return commandStatus


class TestHardware(graphene.Mutation):
    """
    Tests if the hardware is present and talking.
    """

    class Arguments:
        test = TestEnum(required=True)

    Output = TestResults

    def mutate(self, info, test):

        test_output = {}
        success = True
        errors = []
        if test == 0:  # INTEGRATION test
            for module in MODULES:
                try:
                    mcu = mcu_api.MCU(address=MODULES[module]['address'])
                    out = mcu.read_telemetry(
                        module=module,
                        fields=['firmware_version'])
                    mcu_out = {module: out}
                    test_output.update(mcu_out)
                except Exception as e:
                    success = False
                    errors.append(
                        'Error with module : {} : {}'.format(module, e))

        else:
            raise NotImplementedError("Test type not implemented.")

        testResults = TestResults(
            errors=errors,
            success=success,
            results=test_output
        )

        return testResults


class Mutation(graphene.ObjectType):
    """
    Creates mutation endpoints exposed by graphene.
    """
    ping = Ping.Field()
    noop = Noop.Field()
    passthrough = Passthrough.Field()
    testHardware = TestHardware.Field()


schema = graphene.Schema(query=Query, mutation=Mutation)
