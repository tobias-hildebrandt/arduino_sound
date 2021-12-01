desktop: parse player_test
	scripts/desktop_compile_commands.sh

parse: compile_commands_dir build/player.o build/note.o
	clang build/player.o build/note.o src/generator/parse.c -lSDL2 -lm -o build/parse -Wall -Werror -MJ build/compilecommands/parse.json

player_test: compile_commands_dir build/player.o build/note.o
	clang build/player.o build/note.o src/desktop_player/player_test.c -lSDL2 -lm -o build/player -Wall -Werror -MJ build/compilecommands/player.json

build/player.o: compile_commands_dir
	clang src/desktop_player/player.c -fPIC -Wall -Werror -MJ build/compilecommands/player.o.json -c -o build/player.o

build/note.o: compile_commands_dir
	clang src/include/note.c -fPIC -Wall -Werror -MJ build/compilecommands/note.o.json -c -o build/note.o

compile_commands_dir:
	mkdir -p build/compilecommands/

build_arduino:
	# will overwrite compile_command.sh
	scripts/build_arduino.sh "src/ard_sound/" "build"

upload_arduino:
	scripts/upload_arduino.sh "build"