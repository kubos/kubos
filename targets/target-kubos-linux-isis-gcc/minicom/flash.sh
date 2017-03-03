#!/bin/bash
# Copyright (C) 2017 Kubos Corporation
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

spinner() {
    local sp i n
    sp='/-\|'
    n=${#sp}
    while sleep 0.15; do
        printf "%s\b" "${sp:i++%n:1}"
    done
}

start=$(date +%s)

this_dir=$(cd "`dirname "$0"`"; pwd)
project=$(cd "../.."; pwd)
project=$(basename $project)
program=$1
name=$(basename $1)
is_upgrade=0

password=$(cat yotta_config.json | python -c 'import sys,json; x=json.load(sys.stdin); print x["system"]["password"]')

unamestr=`uname`

if [[ "$unamestr" =~ "Linux" ]]; then
    device=`lsusb -d '0403:'`
fi

if [[ "$device" =~ "6001" ]]; then
    echo "Compatible FTDI device found"
    spinner &
    spinner_pid=$!
    disown
else
    echo "No compatible FTDI device found" 1>&2
    exit 0
fi

if [ "$password" == "Kubos123" ]; then
    echo "Using default password"
fi

if [[ "$name" = *.itb ]]; then
    path="/upgrade"
    is_upgrade=1
else if [[ "$name" != "$project" ]]; then
    path="/home/usr/local/bin"
else
    path="/home/usr/bin"
fi

# Minicom doesn't allow any pass-through arguments, so instead we need to 
# generate a script for it to run.
cat > send.tmp <<-EOF
verbose on
send root
expect {
    "Password:" break
    timeout 1 break
}
send $password
expect {
    "~ #" break
    timeout 5 goto end
}
send "cd $path"
send "rm $name"
send "rz -bZ"
! sz -b --zmodem $1
if $is_upgrade = 1 send "fw_setenv kubos_updatefile $name"
send "exit"
end:
! killall minicom -q
EOF

# Run the transfer script
echo "Sending $name to $path on board..."
minicom kubos -o -S send.tmp > flash.log

# Check transfer result
if grep -q incomplete flash.log; then
    echo "Transfer Failed" 1>&2
elif grep -q complete flash.log; then
    echo "Transfer Successful"
elif grep -q incorrect flash.log; then
    echo "Transfer Failed: Invalid password" 1>&2
else
    echo "Transfer Failed: Connection failed" 1>&2
fi

# Cleanup
rm send.tmp
stty sane
kill $spinner_pid

# Print exec time
end=$(date +%s)
runtime=$(expr ${end} - ${start})
echo "Execution time: $runtime seconds"
