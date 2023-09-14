#!/bin/bash

# Default values
create_only=false

while [[ $# -gt 0 ]]; do
  case "$1" in
    -open)
      create_only=true
      shift
      ;;
    *)
      echo "Usage: $0 [-open]"
      exit 1
      ;;
  esac
done

mkdir -p "$(dirname "$0")/devlog"

filename="devlog-$(date +%H:%M:%S-%d.%m.%y).txt"

fullpath="$(dirname "$0")/devlog/$filename"

touch "$fullpath"
echo "created $filename at $fullpath"

if [ "$create_only" = false" ]; then
  nvim "$fullpath"
fi
