#!/bin/bash
CONTAINER=kubos-inst-${TARGET%*@*}

APPS=examples/kubos-rt-example

for APP in $APPS; do
    docker exec -i $CONTAINER bash -c "cd /build/$APP && yotta build -d -- -v"
done
