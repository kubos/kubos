#!/bin/bash

# Start D-Bus system daemon
dbus-daemon --config-file=/etc/dbus-1/kubos.conf --fork --nopidfile --address=unix:path=/tmp/kubos

# Export D-bus variables
export DBUS_SESSION_BUS_ADDRESS=unix:path=/tmp/kubos
export DBUS_STARTER_BUS_TYPE=session
