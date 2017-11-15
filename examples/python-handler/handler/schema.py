#!/usr/bin/env python

# Copyright 2017 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.

"""
Graphene schema setup to enable queries.
"""

import graphene
from models import Subsystem

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

schema = graphene.Schema(query=Query)
