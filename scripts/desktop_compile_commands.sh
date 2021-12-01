#!/bin/bash
sed -e '1s/^/[\'$'\n''/' -e '$s/,$/\'$'\n'']/' build/compilecommands/*.o.json > compile_commands.json