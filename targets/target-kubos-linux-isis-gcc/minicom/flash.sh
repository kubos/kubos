#!/bin/bash
# Copyright (C) 2016 Kubos Corporation

spinner() {
    local i sp n
    sp='/-\|'
    n=${#sp}
    while sleep 0.15; do
        printf "%s\b" "${sp:i++%n:1}"
    done
}

start=$(date +%s.%N)

this_dir=$(cd "`dirname "$0"`"; pwd)
program=$1
name=$(echo $1 | cut -d '/' -f 2)
cmd=$2

unamestr=`uname`

if [[ "$unamestr" =~ "Linux" ]]; then
    device=`lsusb -d '0403:'`
fi

if [[ "$device" =~ "6001" ]]; then
    echo "Compatible FTDI device found"
    echo "Sending file to board..."
    spinner &
    spinner_pid=$!
    disown
else
    echo "No compatible FTDI device found"
    exit 0
fi

# Minicom doesn't allow any pass-through arguments, so instead we need to 
# generate a script for it to run.
# To-Do: Pass-through the root password.  There should be an update to 
# make it more secure than 'password' at some point, likely to be set by
# the user.
cat > send.tmp <<-EOF
verbose on
send root
expect {
    "Password:" break
    timeout 1 break
}
send password
send "cd /usr/bin"
send "rm $name"
send "rz -bZ"
! sz -b --zmodem $1
send "exit"
! killall minicom -q
EOF

# Run the transfer script
minicom kubos -o -S send.tmp > /dev/null
echo "Transfer completed successfully"

# Cleanup
rm send.tmp
stty sane
kill $spinner_pid

# Print exec time
end=$(date +%s.%N)
runtime=$(python -c "print(${end} - ${start})")
echo "Execution time: $runtime seconds"
