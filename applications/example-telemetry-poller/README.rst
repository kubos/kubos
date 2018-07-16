Python Telemetry Poller Demo
====================

This is a barebones demo of a telemetry poller implemented in Python.

This poller sends a graphql query to the payload, iterates over the date sent
back, and sends a mutation to the telemetry-handler for each subsystem/parameter.

The polling interval is set as the second command line argument.
