#!/bin/bash

this_dir=$(cd "`dirname "$0"`"; pwd)

docker build -t kubostech/build -f "$this_dir/Dockerfile.build" "$this_dir"
docker build -t kubostech/ci-build -f "$this_dir/Dockerfile.ci-build" "$this_dir"
