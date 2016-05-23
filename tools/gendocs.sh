#!/bin/bash

rm -rf kubos_docs
maindir=`pwd`
doxygen docs/Doxyfile && mv html kubos_docs

cd kubos-core; doxygen docs/Doxyfile
cd $maindir
mv kubos-core/html kubos_docs/kubos-core

cd libcsp; doxygen docs/Doxyfile
cd $maindir
mv libcsp/html kubos_docs/libcsp

cd freertos/os; doxygen docs/Doxyfile
cd $maindir
mv freertos/os/html kubos_docs/freertos

cd hal/kubos-hal; doxygen docs/Doxyfile
cd $maindir
mv hal/kubos-hal/html kubos_docs/kubos-hal

cd hal/kubos-hal-stm32f407vg; doxygen docs/Doxyfile
cd $maindir
mv hal/kubos-hal-stm32f407vg/html kubos_docs/kubos-hal/kubos-hal-stm32f407vg

cd hal/kubos-hal-msp430f5529; doxygen docs/Doxyfile
cd $maindir
mv hal/kubos-hal-msp430f5529/html kubos_docs/kubos-hal/kubos-hal-msp430f5529
