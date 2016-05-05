#!/bin/bash
CONTAINER=kubos-inst-${TARGET%*@*}

docker exec -i $CONTAINER bash -c "cd /build/$APP && yotta build -d -- -v"
