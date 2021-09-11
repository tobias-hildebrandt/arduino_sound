
parse:
	gcc src/generator/parse.c -o build/parse

build_arduino:
	scripts/build_arduino.sh "src/ard_sound/" "build"

upload_arduino:
	scripts/upload_arduino.sh "build"