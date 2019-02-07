#!/bin/bash

set -e

# Run clippy linter
cargo clippy -- -A clippy::implicit_hasher -A clippy::type_complexity -A clippy::cyclomatic_complexity
