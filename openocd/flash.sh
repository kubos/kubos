#!/bin/bash

unamestr=`uname`

if [[ "$unamestr" =~ "Linux" ]]; then
	device=`lsusb -d '0483:'`
elif [[ "$unamestr" =~ "Darwin" ]]; then
	device=`python $1/osxusb.py -d '0483:'`
fi

if [[ "$device" =~ "3748" ]]; then
	cfg="stm32f407vg.cfg"
fi

if [[ "$device" =~ "374b" ]]; then
	cfg="stm32f407g-disc1.cfg"
fi


if [[ ! -z $cfg ]]; then
	openocd -f $1/$cfg -c "$2"
else
	echo "No compatible ST-Link device found"
fi
