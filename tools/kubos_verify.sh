#!/bin/bash

# This script is intended to verify an environment`s ability
# to compile, run and test the core KubOS services and libraries.

cargo test --package kubos-app-service
cargo test --package telemetry-service
cargo test --package monitor-service
cargo test --package shell-service
cargo test --package file-service

python3 hal/python-hal/i2c/test_i2c.py

pushd apis/app-api/python
poetry run python3 test_app_api.py
popd
