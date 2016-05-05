#!/bin/bash
CONTAINER=kubos-inst-${TARGET%*@*}

APPS=examples/kubos-rt-example
BUILD=$WORKSPACE/jenkins-build

if [[ -d "$BUILD" ]]; then
    rm -rf "$BUILD"
fi

for APP in $APPS; do
    docker exec -i $CONTAINER bash -c "cd /build/$APP && yotta build -d -- -v"
    mkdir -p "$BUILD/$APP"
    cp -r $APP/build/* "$BUILD/$APP"
done
