#!/bin/bash
CONTAINER=kubos-inst-${TARGET%*@*}

docker stop $CONTAINER
docker wait $CONTAINER
docker rm $CONTAINER
