#!/bin/bash
shopt -s globstar
# for file in build/compilecommands/**/*.json; do
#     echo $file
# done
sed -e '1s/^/[\'$'\n''/' -e '$s/,$/\'$'\n'']/' build/compilecommands/**/*.json > compile_commands.json