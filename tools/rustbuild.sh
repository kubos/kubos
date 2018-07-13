#!/bin/bash

# Link in yotta modules for those rust
# modules that need them
./tools/kubos_link.py

# Quick compile check
cargo kubos -c check

# Run all unit tests
cargo kubos -c test
