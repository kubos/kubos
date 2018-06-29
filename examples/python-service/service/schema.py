#!/usr/bin/env python

# Copyright 2017 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.

"""
Graphene schema setup to enable queries.
"""

import graphene
from models import Status, Subsystem

# Local subsystem instance for tracking state
# May not be neccesary when tied into actual hardware
_subsystem = Subsystem(power_on=False)


class Query(graphene.ObjectType):
    """
    Creates query endpoints exposed by graphene.
    """

    subsystem = graphene.Field(Subsystem)

    def resolve_subsystem(self, info):
        """
        Handles request for subsystem query.
        """

        _subsystem.refresh()
        return _subsystem


class PowerOn(graphene.Mutation):
    """
    Creates mutation for Subsystem.PowerOn
    """

    class Arguments:
        power = graphene.Boolean()

    Output = Status

    def mutate(self, info, power):
        """
        Handles request for subsystem powerOn mutation
        """

        status = Status(status=True, subsystem=_subsystem)
        if power is not None:
            status = _subsystem.set_power_on(power)

        return status


class Mutation(graphene.ObjectType):
    """
    Creates mutation endpoints exposed by graphene.
    """

    power_on = PowerOn.Field()


schema = graphene.Schema(query=Query, mutation=Mutation)
