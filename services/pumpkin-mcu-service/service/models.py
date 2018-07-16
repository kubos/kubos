#!/usr/bin/env python

# Copyright 2018 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.

"""
Graphene ObjectType classes for PumpkinMCU Command Status.
"""

import graphene


class CommandStatus(graphene.ObjectType):
    """
    Model representing execution status. This allows us to return
    the status of the mutation function alongside the state of
    the model affected.
    """

    status = graphene.Boolean()
    command = graphene.String()


class TestResults(graphene.ObjectType):
    """
    Model representing Test status.
    Returns status of the hardware as well as status of the
    mutation function.
    """
    errors = graphene.String()
    status = graphene.Boolean()
    results = graphene.JSONString()


class TestEnum(graphene.Enum):
    """
    Enum for denoting test type.
    Currently only Integration is supported.
    """
    INTEGRATION = 0
