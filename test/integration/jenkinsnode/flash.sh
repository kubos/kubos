#!/bin/bash
#
# Kubos Continuous Integration
# Copyright (C) 2016 Kubos Corporation
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#
# Must run as root!
#


this_dir=$(cd "`dirname "$0"`"; pwd)
openocd=$1
cmd=$2
file=$3
search_path=$4
unamestr=`uname`

echo "OpenOCD is: $1"
echo "CMD is: $2"
echo "file is: $3"
echo "search_path is: $4"

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
    export LD_LIBRARY_PATH=$KUBOS_LIB_PATH:$LD_LIBRARY_PATH
    echo $openocd -f $this_dir/$cfg -s $search_path -c \"$cmd $file \"
    $openocd -f $this_dir/$cfg -s $search_path -c "$cmd $file"
    echo "Return value: "
    echo $?

else
    echo "No compatible ST-Link device found"

fi
