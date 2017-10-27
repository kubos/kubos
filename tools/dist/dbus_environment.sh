#!/bin/bash

# Start D-Bus system daemon
dbus-daemon --config-file=/etc/dbus-1/kubos.conf --fork --nopidfile --address=unix:path=/tmp/kubos
