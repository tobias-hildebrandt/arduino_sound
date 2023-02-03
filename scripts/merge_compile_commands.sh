#!/bin/bash
shopt -s globstar

# make sure jq is installed
if ! command -v jq > /dev/null; then
    echo "jq not found, not merging compilation databases"
    exit
else
    echo ok
fi

# make sure overall dir exists
mkdir -p build/compilecommands/

# desktop compilation database
DESKTOP_FILE=build/compilecommands/desktop.json

# start desktop file
echo "[" > $DESKTOP_FILE

# if we can actually merge some files
if [ -d build/desktop/ ]; then
    # copy all fragments into file
    for file in build/desktop/**/*.json; do
        cat "$file" >> $DESKTOP_FILE
    done

    # remove newline and trailing comma (2 bytes)
    truncate -s-2 $DESKTOP_FILE
fi

# end file
printf "\n]" >> $DESKTOP_FILE

# merge desktop and arduino compilation database via jq
jq -s '[.[][]]' build/compilecommands/{arduino,desktop}.json > build/compilecommands/all.json

# remove old compile_commands if it exists
rm -f compile_commands.json

# symlink it
ln -s build/compilecommands/all.json compile_commands.json

