#!/bin/sh

if [ -z "$1" ] || [ -z "$2" ]; then
  echo "first argument must be location of source directory"
  echo "second argument must be location of build directory"
  exit
fi

SOURCE=$(realpath "$1")
BUILD=$(realpath "$2")

cd "$SOURCE" || (echo "error changing to source directory"; exit) # change to source directory

HEADER="clangd_arduino.h"

echo "---trim start"
for file in *.ino; do # attempt to trim each file in directory
  echo "trimming $file"
  # shellcheck disable=SC2016
  sed -i -E "s/(.*$HEADER.*)/\/\/\1/g" "$file" # add "//" at beginning of line
done
echo "---trim done"

echo "---build start"
if arduino-cli compile -b arduino:avr:uno --build-path "$BUILD" --libraries="../" --warnings="all"; then
  echo 'build success!'
else
  echo 'build fail'
fi
echo "---build done"

echo "---untrim start"
for file in *.ino; do
  echo "un-trimming $file"
  # shellcheck disable=SC2016
  sed -i -E "s/\/*(.*$HEADER.*)/\1/g" "$file" # remove all /'s from beginning of line
done
echo "---untrim done"