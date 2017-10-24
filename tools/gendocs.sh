#!/bin/bash
# tools/gendocs.sh
# This script generates the documentation for the kubos source tree
# The script expects two parameters
# - An absolute path to a directory to put the documentation in
# - A version number (typcially represents the current release version)
# usage: ./tools/gendocs.sh /absolute/path/to/output/dir x.x.x

this_dir=$(cd "`dirname "$0"`"; pwd)
kubos_dir=$(cd "$this_dir/.."; pwd)
out_dir=$1
release_ver=$2

if [ "$out_dir" = "" ]; then
    echo "Error: required output directory missing"
    exit 1
fi

if [ ! -d "$out_dir" ]; then
    echo "Error: output directory does not exist"
    exit 1
fi

if [ "$release_ver" = "" ]; then
    echo "Error: required release version missing"
    exit 1
fi

run_cmd() {
    echo "$@"
    $@
}

gendocs() {
    dir=$1
    doxyfile=$2
    version=$3

    cd $dir
    if [ "$version" != "" ]; then
        echo "( cat $doxyfile ; echo \"PROJECT_NUMBER=$version\" ) | doxygen -"
        ( cat $doxyfile ; echo "PROJECT_NUMBER=$version" ) | doxygen -
    else
        run_cmd doxygen $doxyfile
    fi
}

gendocs_yt_module() {
    module_dir=$1
    out_relpath=$2

    run_cmd cd $module_dir
    module_version=$(yt version 2>&1 | awk '{ print $2 }')

    gendocs $module_dir docs/Doxyfile $module_version
    run_cmd mv $module_dir/html $out_dir/$out_relpath
}

gendocs $kubos_dir docs/Doxyfile $release_ver
run_cmd mv $kubos_dir/html/* $out_dir

gendocs_yt_module $kubos_dir/kubos-core kubos-core
gendocs_yt_module $kubos_dir/libcsp libcsp
gendocs_yt_module $kubos_dir/freertos/os freertos
gendocs_yt_module $kubos_dir/hal/kubos-hal kubos-hal
gendocs_yt_module $kubos_dir/hal/kubos-hal-iobc kubos-hal/kubos-hal-iobc
gendocs_yt_module $kubos_dir/hal/kubos-hal-linux kubos-hal/kubos-hal-linux
gendocs_yt_module $kubos_dir/hal/kubos-hal-stm32f4 kubos-hal/kubos-hal-stm32f4
gendocs_yt_module $kubos_dir/hal/kubos-hal-msp430f5529 kubos-hal/kubos-hal-msp430f5529
gendocs_yt_module $kubos_dir/radio/radio-api radio-api
gendocs_yt_module $kubos_dir/telemetry telemetry
gendocs_yt_module $kubos_dir/telemetry-aggregator telemetry-aggregator
gendocs_yt_module $kubos_dir/telemetry-storage telemetry-storage
gendocs_yt_module $kubos_dir/ipc ipc
