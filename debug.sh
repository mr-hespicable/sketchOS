#!/bin/bash

cargo build
cargo run -- -s -S &
sleep 2 #wait a bit for thing to run and stuff
gdb -ex 'file target/x86_64-sketchOS/debug/sketch_os' -ex 'tar ext :1234'           -ex 'dashboard -layout breakpoints expressions history memory source threads variables'            -ex 'tui enable' -ex 'b _start' -ex 'c'

PID=$(pgrep qemu)
kill $PID
sleep 0.5
clear
