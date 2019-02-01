#!/bin/bash

set -e

# Link in yotta modules for those rust
# modules that need them
./tools/kubos_link.py

# Run clippy linter
cargo kubos -c clippy -- -- -A clippy::implicit_hasher -A clippy::type_complexity -A clippy::cyclomatic_complexity
