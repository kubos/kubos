#!/bin/bash

echo WORKSPACE=$WORKSPACE
echo PWD=$PWD
echo TARGET=$TARGET
CONTAINER=kubos-inst-${TARGET%*@*}

docker version
docker images

docker pull kubostech/ci-build || exit $?

./repo init -u git://github.com/kubostech/kubos-manifest || exit $?
./repo sync || exit $?

# make sure the container isn't still running and is deleted
(docker stop $CONTAINER && docker wait $CONTAINER) || echo
docker rm $CONTAINER || echo

docker run --name $CONTAINER -d=True -v "$WORKSPACE:/build" kubostech/ci-build
docker exec -i $CONTAINER python tools/yotta_link.py
docker exec -i $CONTAINER yotta target --global $TARGET
