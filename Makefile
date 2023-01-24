PARSER=parse
PLAYER=player

OUT_DIR=out
OUT_FILE=out.h
BUILD=build
COMPILECOMMANDSDIR=compilecommands

INC=-I./src/include

CFLAGS=-Wall -Werror -g -D_GNU_SOURCE $(COMPCOMFLAG)
COMPCOMFLAG=-MJ build/compilecommands/
CC=clang

SDL=-lSDL2
LIBS=-lm

usage:
	echo "Make tasks: desktop, build_arduino, upload_arduino, clean"

# use dependencies to build necessary binaries, then run script on compile_commands.json
desktop: $(PARSER)
	scripts/desktop_compile_commands.sh

# link parser binary
$(PARSER): $(BUILD)/desktop_player/player.o $(BUILD)/include/note.o $(BUILD)/generator/parse.o
	$(CC) $(CFLAGS) $(LIBS) $(SDL) $(INC) $^ -o $(BUILD)/$@

# build single source code file without linking
$(BUILD)/%.o: src/%.c
	mkdir -p $(dir $@)
	mkdir -p $(dir build/compilecommands/$(subst $(BUILD)/,,$@))
	$(CC) $(CFLAGS) $(INC) -c $< -o $@ $(COMPCOMFLAG)$(subst $(BUILD)/,,$@).json

# run the parser on an abc file
%.abc: desktop
	$(BUILD)/$(PARSER) $@ $(OUT_DIR)/$(OUT_FILE)

build_arduino:
	# will overwrite compile_commands.json
	scripts/build_arduino.sh "src/ard_sound/" "build"

upload_arduino:
	scripts/upload_arduino.sh "build"

.PHONY: clean
clean:
	rm -r $(BUILD)
