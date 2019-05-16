#!/usr/bin/env python

# Copyright 2018 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.

"""
Graphene schema setup to enable queries.
"""

import graphene
import logging
from .models import *
import mcu_api

# Initialize MODULES global. This is then configured in the service file.
# "module_name" must match an entry in the API configuration dict
# "address" must be a valid I2C address int.
MODULES = {
    "module_name": {"address": 0xFF}
}

logger = logging.getLogger("pumpkin-mcu-service")

class Query(graphene.ObjectType):
    """
    Creates query endpoints exposed by graphene.
    """
    ping = graphene.String()
    moduleList = graphene.JSONString()
    fieldList = graphene.List(graphene.String, module=graphene.String())
    read = graphene.String(
        module=graphene.String(),
        count=graphene.Int())
    mcuTelemetry = graphene.JSONString(
        module=graphene.String(),
        fields=graphene.List(graphene.String, default_value=["all"]))

    def resolve_ping(self, info):
        return "pong"

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
        try:
            bin_data = mcu.read(count=count)

            return bin_data.encode("hex")
        except Exception as e:
            logger.error("Failed to read {} bytes from {}: {}".format(count, module, e))
            raise

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
        fields = list(map(str, fields))
        mcu = mcu_api.MCU(address=address)
        try:
            out = mcu.read_telemetry(module=module, fields=fields)
            return out
        except Exception as e:
            logger.error("Failed to read telemetry from {}: {}".format(module, e))
            raise


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
        if type(command) == str:
            command = str.encode(command)
        mcu = mcu_api.MCU(address=MODULES[module]['address'])
        try:
            out = mcu.write(command)
            commandStatus = CommandStatus(status=out[0], command=out[1])
            return commandStatus
        except Exception as e:
            logger.error("Failed to send passthrough to {}: {}".format(module, e))
            raise


class Test(graphene.Mutation):
    """
    Tests the service and hardware is present and talking.
    """

    class Arguments:
        test = TestEnum(required=True)

    Output = TestResults

    def mutate(self, info, test):

        success = True
        errors = []
        test_output = {}
        if test == 0:  # PING
            test_output = "pong"
        elif test == 1:  # NOOP
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
                    msg = 'Error with module : {} : {}'.format(module, e)
                    logger.error(msg)
                    errors.append(msg)

        elif test == 2:  # INTEGRATION test
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
                    msg = 'Error with module : {} : {}'.format(module, e)
                    logger.error(msg)
                    errors.append(msg)
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
    passthrough = Passthrough.Field()
    test = Test.Field()


schema = graphene.Schema(query=Query, mutation=Mutation)
