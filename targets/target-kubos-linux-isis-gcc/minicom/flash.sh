#!/bin/bash
# Copyright (C) 2016 Kubos Corporation

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
else
    echo "No compatible FTDI device found"
    exit 0
fi

# Setup serial connection configuration file
# To-Do: Detect name of USB device to use (in case it's not USB0)
cat > /etc/minicom/minirc.kubos <<-EOF
pu port         /dev/ttyUSB0
pu baudrate     115200
pu bits         8
pu parity       N
pu stopbits     1
pu rtscts       no
EOF

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
! killall -9 minicom
EOF

# Run the transfer script
minicom kubos -o -S send.tmp

# Cleanup
rm send.tmp
rm /etc/minicom/minirc.kubos
stty sane
