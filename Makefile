PARSER=parse

SRC_DIR=c_src
OUT_DIR=out
OUT_FILE=out.h
BUILD=build
COMPILE_COMMANDS_DIR=compilecommands
DESKTOP_BUILD=$(BUILD)/desktop/
ARDUINO_BUILD=$(BUILD)/arduino/

INC=-I./$(SRC_DIR)/include

CFLAGS=-Wall -Werror -g -D_GNU_SOURCE
COMPCOMFLAG=-MJ
CC=clang

SDL=-lSDL2
LIBS=-lm

usage:
	@echo "Make tasks: <some_file>.abc, desktop, build_arduino, upload_arduino, clean"

# use dependencies to build necessary binaries, then run script on compile_commands.json
desktop: $(PARSER)
	scripts/merge_compile_commands.sh

# link parser binary
$(PARSER): $(DESKTOP_BUILD)/desktop_player/player.o $(DESKTOP_BUILD)/include/note.o $(DESKTOP_BUILD)/generator/parse.o
	$(CC) $(CFLAGS) $(LIBS) $(SDL) $(INC) $^ -o $(DESKTOP_BUILD)/$@

# build single source code file without linking
# also output a json compilation database for file (stripping out the .o)
$(DESKTOP_BUILD)/%.o: $(SRC_DIR)/%.c
	mkdir -p $(dir $@)
	$(CC) $(CFLAGS) $(INC) -c $< -o $@ $(COMPCOMFLAG) $(subst .o,,$@).json

# run the parser on an abc file
%.abc: desktop
	mkdir -p $(OUT_DIR)
	$(DESKTOP_BUILD)/$(PARSER) $@ "$(OUT_DIR)/$(OUT_FILE)"

build_arduino:
	mkdir -p $(BUILD)/$(COMPILE_COMMANDS_DIR)
	scripts/build_arduino.sh "$(SRC_DIR)/ard_sound/" $(ARDUINO_BUILD)
	scripts/merge_compile_commands.sh

upload_arduino:
	scripts/upload_arduino.sh $(ARDUINO_BUILD)

.PHONY: clean
clean:
	rm -rf $(BUILD) $(OUT_DIR)
