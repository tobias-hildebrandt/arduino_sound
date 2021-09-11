#!/bin/sh

if [ -z "$1" ]; then
  echo "first argument must be location of build directory"
fi

echo "---upload start"
for device in /dev/ttyACM*; do
    echo "attempting upload to $device"
    if arduino-cli upload -b arduino:avr:uno --input-dir "$1" -p "$device"; then
        echo "upload successful!"
        break
    else
        echo "upload failed"
    fi
done
echo "---upload done"
echo