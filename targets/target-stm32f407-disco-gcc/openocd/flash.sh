#!/bin/bash

this_dir=$(cd "`dirname "$0"`"; pwd)
program=$1 # program is fist becasue for debugging there is not a command provided
           # openocd will start a gdb server by default if there's not a command provided
cmd=$2
if [[ ! -z $cmd ]]; then
    openocd_arg="$cmd $program"
fi
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
    if [[ ! -z $openocd_arg ]]; then #Flashing the target
        echo openocd -f $this_dir/$cfg -c \"$openocd_arg\"
        openocd -f $this_dir/$cfg -c "$openocd_arg"
    else                            #Debugging the Target
        (openocd -f $this_dir/$cfg -c "$openocd_arg" &) #Start the gdb server in a sub-shell
        sleep 3
        arm-none-eabi-gdb -ex "target remote localhost:3333" -ex "file $program"
    fi
else
    echo "No compatible ST-Link device found"
fi



