#!/bin/bash
set -e

exec_cmd() {
  echo $@
  $@
}

docker_run() {
  exec_cmd docker run -it --rm -v $PWD:/build -w /build $IMAGE $@
}

docker_yotta() {
  docker_run yotta --verbose $@
}

docker_yotta target $TARGET
docker_yotta build -d -- -v

if [[ "$TESTS" = "1" ]]; then
  docker_yotta test
fi
