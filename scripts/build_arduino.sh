#!/bin/sh

# verify arguments
if [ -z "$1" ] || [ -z "$2" ]; then
    echo "first argument must be location of source directory"
    echo "second argument must be location of build directory"
    exit
fi

BASE=$(pwd)
SOURCE=$(realpath "$1")
BUILD=$(realpath "$2")

# change to source directory
cd "$SOURCE" || (echo "error changing to source directory"; exit)

# invoke arduino-cli
echo "---build start"
if arduino-cli compile -b arduino:avr:uno --build-path "$BUILD" --libraries="../" --warnings="all"; then
    echo '---build success!'
else
    echo '---build fail'
    exit 1
fi

# set up arduino compile_commands
cd "$BASE" || (echo "error changing to base directory"; exit)

# arduino-cli has a static output for its compilation database, so move it
mv "${BUILD}/compile_commands.json" "${BUILD}/../compilecommands/arduino.json"

# fix source path of ino, since arduino-cli makes a copy
sed -i 's/build\/arduino\/sketch\/ard_sound.ino.cpp\"/c_src\/ard_sound\/ard_sound.ino\"/g' "${BUILD}/../compilecommands/arduino.json"

