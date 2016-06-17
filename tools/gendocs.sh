#!/bin/bash
this_dir=$(cd "`dirname "$0"`"; pwd)
kubos_dir=$(cd "$this_dir/.."; pwd)
out_dir=`pwd`"/"$1

if [ "$out_dir" = "" ]; then
    echo "Error: required output directory missing"
    exit 1
fi

if [ ! -d "$out_dir" ]; then
    echo "Error: output directory does not exist"
    exit 1
fi

doxygen $kubos_dir/docs/Doxyfile
mv $kubos_dir/html/* $out_dir

cd $kubos_dir/kubos-core
doxygen docs/Doxyfile
mv $kubos_dir/kubos-core/html $out_dir/kubos-core

cd $kubos_dir/libcsp
doxygen docs/Doxyfile
mv $kubos_dir/libcsp/html $out_dir/libcsp

cd $kubos_dir/freertos/os
doxygen docs/Doxyfile
mv $kubos_dir/freertos/os/html $out_dir/freertos

cd $kubos_dir/hal/kubos-hal
doxygen docs/Doxyfile
mv $kubos_dir/hal/kubos-hal/html $out_dir/kubos-hal

cd $kubos_dir/hal/kubos-hal-stm32f4
doxygen docs/Doxyfile
mv $kubos_dir/hal/kubos-hal-stm32f4/html $out_dir/kubos-hal/kubos-hal-stm32f4

cd $kubos_dir/hal/kubos-hal-msp430f5529
doxygen docs/Doxyfile
mv $kubos_dir/hal/kubos-hal-msp430f5529/html $out_dir/kubos-hal/kubos-hal-msp430f5529
