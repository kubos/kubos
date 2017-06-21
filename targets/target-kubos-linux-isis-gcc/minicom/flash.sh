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
# Flash files to ISIS OBC board
#

##########################################################################
# Scrape transfer status from minicom log. Looks like this:
# "Bytes Sent: 693248/1769379 BPS:8343 ETA 02:08"
##########################################################################
progress() {
    
    local line

    while sleep 1; do
        line=$(grep -m 1 -o -e "Bytes Sent.*\e" -e "Bytes Sent.*     " flash.log)
        printf "\033[2K${line}\r"
    done     
    
}

##########################################################################
# Minicom doesn't allow any pass-through arguments, so instead we need to 
# generate a script for it to run.
# The generated script will:
#     Navigate to the correct path
#     Delete any previous versions of the file
#     Delete any previous init scripts (if flashing application)
#     Check that there is enough room for the file at the destination
#     Flash the file
#     Update the kubos_updatefile variable (if flashing upgrade package)
#     Re/start the application (if flashing application and boot desired)
#
# Globals:
#     password
#     is_upgrade
#     is_app
#     is_run
# Arguments:
#     path of file to flash
#     path to flash to
# Returns:
#     none
##########################################################################
create_send_script() {

    local path=$1
    local name
    local size
    
    name=$(basename ${path})
    size=$(du --apparent-size ${path} | cut -f1) 
    
    echo "Sending ${name} (${size} 1k blocks) to $2 on board..."
   
    # The script code here must ignore the current indentation level and 
    # instead starts from a plain left-alignment. Script is made up of 
    # 'runscript' commands
    cat > send.tmp <<-EOF
verbose on
send root
expect {
    "Password:" break
    timeout 1 break
}
send ${password}
expect {
    "~ #" break
    timeout 1 goto end
}
timeout 3600
send "mkdir -p $2"
send "cd $2"
if ${is_upgrade} = 1 goto check_space
send "rm -f ${name}"
if ${is_app} = 1 send "rm -f /home/system/etc/init.d/S*${name}"

check_space:
expect {
    "$2 #" break
    timeout 1
}
send "df $2 | grep -o '/.*%' | grep -Eo '[0-9]+ *[0-9]+%' | grep -Eo '\^[0-9]+'"
expect {
    "$2 #" break
    timeout 1
}
! /usr/bin/test ${size} -lt \$(tail flash.log -n2 | grep -m 1 -o '[0-9]*')
if \$? = 0 goto send_file
! echo "Not enough room for file transfer, aborting" >&2
goto exit

send_file:
if ${is_upgrade} = 1 goto send_file_resume
send "rz -bU -w 8192"
! sz -bU -w 8192 ${path}
if ${is_run} = 1 goto start_process
goto exit

send_file_resume:
send "rz -brU -w 8192"
! sz -brU -w 8192 ${path}
if ${is_upgrade} = 1 send "fw_setenv kubos_updatefile ${name}"
if ${is_run} = 0 goto exit

start_process:
send "start-stop-daemon -K -v -p /var/run/${name}.pid"
send "start-stop-daemon -S -v -m -b -p /var/run/${name}.pid --exec $2/${name}"

exit:
send "exit"

end:
! killall minicom -q
EOF

}

##########################################################################
# Generate a default init script for the new application. 
# File name will be 'S{run_level}{app_name}'
# ex. S90MyProj
#
# Globals:
#     app_name
#     init_script
# Arguments:
#     none
# Returns:
#     none
##########################################################################
create_init_script() {

    echo "Creating init script"

    # Delete any previous versions of the init script to avoid clutter
    rm -f S*${app_name}
    
    # The init script code here must ignore the current indentation level
    # and instead starts from a plain left-alignment.
    cat > ${init_script} <<-EOF
#!/bin/sh

NAME=${app_name}
PROG=/home/system/usr/bin/\${NAME}
PID=/var/run/\${NAME}.pid

case "\$1" in
    start)
        echo "Starting \${NAME}: "
        start-stop-daemon -S -q -m -b -p \${PID} --exec \${PROG}
        rc=\$?
        if [ \${rc} -eq 0 ]; then
            echo "OK"
        else
            echo "FAIL" >&2
        fi
        ;;
    stop)
        echo "Stopping \${NAME}: "
        start-stop-daemon -K -q -p \${PID}
        rc=\$?
        if [ \${rc} -eq 0 ]; then
            echo "OK"
        else
            echo "FAIL" >&2
        fi
        ;;
    restart)
        "\$0" stop
        "\$0" start
        ;;
    *)
        echo "Usage: \$0 {start|stop|restart}"
        ;;
esac

exit \${rc}
EOF

    chmod 0755 ${init_script}
}

##########################################################################
# Call the minicom transfer script to flash the file
#
# Globals:
#     none
# Arguments:
#     none
# Returns:
#     0 - Transfer successful
#     1 - Transfer failed
##########################################################################
send_file() {

    # Run the transfer script
    minicom kubos -o -S send.tmp > flash.log
     
    local retval=1
    
    # Check transfer result
    if grep -q incomplete flash.log; then
        echo "Transfer Failed" 1>&2
    elif grep -q complete flash.log; then
        echo "Transfer Successful"
        retval=0
    elif grep -q incorrect flash.log; then
        echo "Transfer Failed: Invalid password" 1>&2 
    elif grep -q "Not enough room" flash.log; then
        echo "Destination is out of space. Please remove files and retry" 1>&2
    else
        echo "Transfer Failed: Connection failed" 1>&2
    fi
    
    return "${retval}"
    
}

##########################################################################
# Main Script
##########################################################################
main() {

    # Protect all the local-only variables (per google style)
    local dest_dir
    local device
    local end
    local file
    local init_boot
    local init_flash
    local name
    local path
    local progress_pid
    local retval
    local run_level
    local runtime
    local start
    local this_dir
    local unamestr
    
    is_app=0
    is_run=0
    is_upgrade=0
    
    start=$(date +%s)
    this_dir=$(cd "`dirname "$0"`"; pwd)
    file=$1
    name=$(basename $1)
       
    password=$(cat yotta_config.json | python -c 'import sys,json; x=json.load(sys.stdin); print x["system"]["password"]')
    dest_dir=$(cat yotta_config.json | python -c 'import sys,json; x=json.load(sys.stdin); print x["system"]["destDir"]')
    init_boot=$(cat yotta_config.json | python -c 'import sys,json; x=json.load(sys.stdin); print x["system"]["initAtBoot"]')
    run_level=$(cat yotta_config.json | python -c 'import sys,json; x=json.load(sys.stdin); print x["system"]["runLevel"]')
    init_flash=$(cat yotta_config.json | python -c 'import sys,json; x=json.load(sys.stdin); print x["system"]["initAfterFlash"]')
    app_name=$(cat ../../module.json | python -c 'import sys,json; x=json.load(sys.stdin); print x["name"]')
    
    init_script="S${run_level}${app_name}"
    
    unamestr=$(uname)
    
    if [[ "${unamestr}" =~ "Linux" ]]; then
        device=$(lsusb -d '0403:')
    fi
    
    if [[ "${device}" =~ "6001" ]]; then
        echo "Compatible FTDI device found"
        progress &
        progress_pid=$!
        disown
    else
        echo "No compatible FTDI device found" 1>&2
        exit 0
    fi
    
    if [[ "${password}" = "Kubos123" ]]; then
        echo "Using default password"
    fi
    
    if [[ "${name}" = *.itb ]]; then
        path="/upgrade"
        is_upgrade=1
    elif [[ "${name}" != "${app_name}" ]]; then
        path="${dest_dir}"
    else
        path="/home/system/usr/bin"
        is_app=1
        if [[ "${run_level}" -gt 99 || "${run_level}" -lt 10 ]]; then
            echo "Run level of ${run_level} outside of range (10-99). Setting to default."
            run_level=50
        fi
        if [[ "${init_flash}" = "True" ]]; then
            echo "Will start application after flash"
            is_run=1
        fi
    fi
    
    # Send the file
    create_send_script ${file} ${path}
    send_file
    retval=$?
    
    # If necessary, send init script
    if [[ "${retval}" = 0 && "${is_app}" = 1 && "${init_boot}" = "True" ]]; then
        is_app=0
        is_run=0
        rm send.tmp
        create_init_script
        create_send_script ${init_script} /home/system/etc/init.d
        send_file
        retval=$?
    fi
    
    # Cleanup
    rm send.tmp
    stty sane
    kill ${progress_pid}
    
    # Print exec time
    end=$(date +%s)
    runtime=$(expr ${end} - ${start})
    echo "Execution time: ${runtime} seconds"
    
    exit ${retval}
    
}

main "$@"
