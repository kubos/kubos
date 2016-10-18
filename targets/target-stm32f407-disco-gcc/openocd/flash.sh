#!/bin/bash

this_dir=$(cd "`dirname "$0"`"; pwd)
cmd=$1
unamestr=`uname`

if [[ "$unamestr" =~ "Linux" ]]; then
	device=`lsusb -d '0483:'`
elif [[ "$unamestr" =~ "Darwin" ]]; then
	device=`python $this_dir/osxusb.py -d '0483:'`
fi

if [[ "$device" =~ "3748" ]]; then
	cfg="stm32f407vg.cfg"
fi

if [[ "$device" =~ "374b" ]]; then
	cfg="stm32f407g-disc1.cfg"
fi


if [[ ! -z $cfg ]]; then
	echo openocd -f $this_dir/$cfg -c \"$cmd\"
	openocd -f $this_dir/$cfg -c "$cmd"
else
	echo "No compatible ST-Link device found"
fi
