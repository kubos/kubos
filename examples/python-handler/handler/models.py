#!/usr/bin/env python

# Copyright 2017 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.

"""
Graphene ObjectType classes for subsystem modeling.
"""

import graphene


class Subsystem(graphene.ObjectType):
    """
    Model encapsulating subsystem functionality.
    """

    power_on = graphene.Boolean()

    def refresh(self):
        """
        Will hold code for refreshing the status of the subsystem
        model based on queries to the actual hardware.
        """

        print "Querying for subsystem status"
        self.power_on = not self.power_on

    def set_power_on(self, power_on):
        """
        Controls the power state of the subsystem
        """

        print "Sending new power state to subsystem"
        self.power_on = power_on
        return Status(status=True, subsystem=self)


class Status(graphene.ObjectType):
    """
    Model representing execution status. This allows us to return
    the status of the mutation function alongside the state of
    the model affected.
    """

    status = graphene.Boolean()
    subsystem = graphene.Field(Subsystem)
