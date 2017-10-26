#!/bin/bash

# Start D-Bus system daemon
dbus-daemon --config-file=./scripts/sess.conf --fork --nopidfile --address=unix:path=/tmp/kubos

# Set D-Bus related environment variables
export DBUS_SESSION_BUS_ADDRESS=unix:path=/tmp/kubos
export DBUS_STARTER_BUS_TYPE=session
