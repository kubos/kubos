#!/bin/bash
set -e

KUBOS_CORE=$(cd "`dirname "$0"`/.."; pwd)

docker_run() {
    WORK_DIR=$1
    shift

    set -x
    docker run -it \
        -v $RIOTBASE:/data/RIOT \
        -v $KUBOS_CORE:/data/kubos-core \
        -w $WORK_DIR \
        riotbuild \
        $@
    set +x
}

docker_make() {
    WORK_DIR=$1
    shift

    docker_run $WORK_DIR make QUIET=$QUIET BOARD=$BOARD $@
}

echo Building for $BOARD
docker_make /data/kubos-core/examples/kubos-shell all

# TODO: setup infra to run unit tests on hardware

if [[ "$TESTS" = "1" ]]; then
    echo Unit tests enabled for $BOARD
    docker_make /data/kubos-core/test-suite all
    docker_run /data/kubos-core/test-suite ./bin/native/kubos-test.elf
fi
