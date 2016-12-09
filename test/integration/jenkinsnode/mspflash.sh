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

KUBOS_LIB_PATH="/usr/local/lib/python2.7/dist-packages/kubos/lib/linux/"

# find mspdebug:
mspdebug=$1

# PROG
cmd=$2

# BINARY
exe=$3

echo "mspdebug is: $1"
echo "CMD is: $2"
echo "binary to upload is: $3"
echo""

export LD_LIBRARY_PATH=$KUBOS_LIB_PATH:$LD_LIBRARY_PATH

echo $mspdebug tilib \"$cmd $exe\" --allow-fw-update
$mspdebug tilib "$cmd $exe" --allow-fw-update

echo "Return value:" 
echo $?
