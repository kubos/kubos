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

progress() {
    while sleep 1; do
        line=$(cat flash.log | grep "Bytes Sent")
        printf "$line\r"
    done   
}

# Minicom doesn't allow any pass-through arguments, so instead we need to 
# generate a script for it to run.
create_send_script() {
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
    timeout 3600
    send "mkdir -p $path"
    send "cd $path"
    send "rm $name"
    send "rz -w 8192"
    ! sz -w 8192 $1
    if $is_upgrade = 1 send "fw_setenv kubos_updatefile $name"
    send "exit"
    end:
    ! killall minicom -q
    EOF
}

create_init_script() {
    cat > S$1$2 <<-EOF
    #!/bin/sh

    NAME=$2
    PROG=/usr/sbin/${NAME}
    PID=/var/run/${NAME}.pid
    
    case "$1" in
        start)
        echo "Starting ${NAME}: "
        start-stop-daemon -S -q -m -b -p ${PID} --exec ${PROG}
        rc=$?
        if [ $rc -eq 0 ]
        then
            echo "OK"
        else
            echo "FAIL" >&2
        fi
        ;;
        stop)
        echo "Stopping ${NAME}: "
        start-stop-daemon -K -q -p ${PID}
        rc=$?
        if [ $rc -eq 0 ]
        then
            echo "OK"
        else
            echo "FAIL" >&2
        fi
        ;;
        restart)
        "$0" stop
        "$0" start
        ;;
        *)
        echo "Usage: $0 {start|stop|restart}"
        ;;
    esac
    
    exit $rc
    EOF
}

# ------ Start of actual script ---------

start=$(date +%s)

this_dir=$(cd "`dirname "$0"`"; pwd)
name=$(basename $1)
is_upgrade=0

password=$(cat yotta_config.json | python -c 'import sys,json; x=json.load(sys.stdin); print x["system"]["password"]')
dest_dir=$(cat yotta_config.json | python -c 'import sys,json; x=json.load(sys.stdin); print x["system"]["destDir"]')
init=$(cat yotta_config.json | python -c 'import sys,json; x=json.load(sys.stdin); print x["system"]["initAtBoot"]')
app_name=$(cat ../../module.json | python -c 'import sys,json; x=json.load(sys.stdin); print x["name"]')

unamestr=`uname`

if [[ "$unamestr" =~ "Linux" ]]; then
    device=`lsusb -d '0403:'`
fi

if [[ "$device" =~ "6001" ]]; then
    echo "Compatible FTDI device found"
    progress &
    progress_pid=$!
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
elif [[ "$name" != "$app_name" ]]; then
    path="$dest_dir"
else
    path="/home/usr/bin"
fi

create_send_script

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
kill $progress_pid

# Print exec time
end=$(date +%s)
runtime=$(expr ${end} - ${start})
echo "Execution time: $runtime seconds"
