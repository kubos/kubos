#!/bin/bash

echo "Setup Environment"
source env.sh

echo "Cleaning build"
rm -rf ~/.xargo
cargo clean

PKG_CONFIG_ALLOW_CROSS=1 RUST_TARGET_PATH=`pwd` xargo build --target thumbv7m-unknown-linux-uclibc -p kubos-app-service --release
PKG_CONFIG_ALLOW_CROSS=1 RUST_TARGET_PATH=`pwd` xargo build --target thumbv7m-unknown-linux-uclibc -p scheduler-service --release
PKG_CONFIG_ALLOW_CROSS=1 RUST_TARGET_PATH=`pwd` xargo build --target thumbv7m-unknown-linux-uclibc -p telemetry-service --release
PKG_CONFIG_ALLOW_CROSS=1 RUST_TARGET_PATH=`pwd` xargo build --target thumbv7m-unknown-linux-uclibc -p shell-service --release
PKG_CONFIG_ALLOW_CROSS=1 RUST_TARGET_PATH=`pwd` xargo build --target thumbv7m-unknown-linux-uclibc -p file-service --release
PKG_CONFIG_ALLOW_CROSS=1 RUST_TARGET_PATH=`pwd` xargo build --target thumbv7m-unknown-linux-uclibc -p monitor-service --release