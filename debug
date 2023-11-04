#!/bin/bash

cargo run -- -s -S &
sleep 2 #make sure that program has actually started
gdb -ex 'file target/x86_64-sketchOS/debug/sketch_os' -ex 'tar ext :1234' -ex 'tui enable' -ex 'b _start'
PID=$(pgrep qemu)
if kill -0 "$PID" 2>/dev/null; then
    kill "$PID"
fi
