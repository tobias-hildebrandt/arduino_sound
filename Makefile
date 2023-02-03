PARSER=parse
PLAYER=player

SRC_DIR=c_src
OUT_DIR=out
OUT_FILE=out.h
BUILD=build
COMPILECOMMANDSDIR=compilecommands

INC=-I./$(SRC_DIR)/include

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
$(BUILD)/%.o: $(SRC_DIR)/%.c
	mkdir -p $(dir $@)
	mkdir -p $(dir build/compilecommands/$(subst $(BUILD)/,,$@))
	$(CC) $(CFLAGS) $(INC) -c $< -o $@ $(COMPCOMFLAG)$(subst $(BUILD)/,,$@).json

# run the parser on an abc file
%.abc: desktop
	mkdir -p $(OUT_DIR)
	$(BUILD)/$(PARSER) $@ "$(OUT_DIR)/$(OUT_FILE)"

# will overwrite compile_commands.json
build_arduino:
	scripts/build_arduino.sh "$(SRC_DIR)/ard_sound/" $(BUILD)

upload_arduino:
	scripts/upload_arduino.sh $(BUILD)

.PHONY: clean
clean:
	rm -r $(BUILD) $(OUT_DIR)
