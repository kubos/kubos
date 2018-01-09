#!/bin/bash
set -e
this_dir=$(cd "`dirname "$0"`"; pwd)

extra_remote=$1

./repo init -u git://github.com/kubos/kubos-manifest
./repo sync

if [ ! -z $extra_remote ]
then
    ./repo forall -c 'git remote rm '$extra_remote'; git remote add '$extra_remote' https://github.com/'$extra_remote'/$REPO_PROJECT'
fi


./tools/kubos_link.py --link --all
