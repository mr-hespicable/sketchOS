#!/bin/bash

cargo build
cargo run -- -s -S &
sleep 2 #wait a bit for thing to run and stuff
gdb -ex 'file target/x86_64-sketchOS/debug/sketch_os' -ex 'tar ext :1234' -ex 'dashboard -layout breakpoints expressions threads variables registers source assembly' -ex 'source debug_files/breaks.txt' -ex 'c'
PID=$(pgrep qemu)
kill $PID
