#!/bin/sh

echo "---trim start"
for file in *.ino; do
  echo "trimming $file"
  # shellcheck disable=SC2016
  sed -i -E 's/(.*scuffed.*)/\/\/\1/g' "$file"
done
echo "---trim done"

echo "---build start"
if arduino-cli compile -b arduino:avr:uno --build-path build/; then
  echo 'build success!'
else
  echo 'build fail'
fi
echo "---build done"

echo "---untrim start"
for file in *.ino; do
  echo "un-trimming $file"
  # shellcheck disable=SC2016
  sed -i -E 's/\/\/(.*scuffed.*)/\1/g' "$file"
done
echo "---untrim done"