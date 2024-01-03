#!/bin/bash

cargo build
cargo run -- -s -S &
sleep 2 #wait a bit for thing to run and stuff
gdb -ex 'file target/x86_64-sketchOS/debug/sketch_os' -ex 'tar ext :1234' -ex 'tui enable' -ex 'b _start'

PID=$(pgrep qemu)
kill $PID
