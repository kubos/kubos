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
