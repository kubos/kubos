#!/bin/bash
#This script starts a gdb server and a gdb instance and runs
#a gdb command file that:
# - Connects to the gdb server
# - Sets the target file to the project executable

mspdebug tilib "gdb 3333" --allow-fw-update &
sleep 5
msp430-gdb -ex "target remote localhost:3333" -ex "file $1"
