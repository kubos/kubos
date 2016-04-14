#!/bin/bash
set -e
this_dir=$(cd "`dirname "$0"`"; pwd)

./repo init -u git://github.com/openkosmosorg/kubos-manifest
./repo sync
