PARSER=parse
PLAYER=player

BUILD=build

CFLAGS=-Wall -Werror
COMPCOMFLAG=-MJ build/compilecommands/
CC=clang

SDL=-lSDL2
LIBS=-lm

desktop: $(PLAYER) $(PARSER)
	scripts/desktop_compile_commands.sh

$(PARSER): $(BUILD)/desktop_player/player.o $(BUILD)/include/note.o
	clang src/generator/parse.c $(CFLAGS) $(LIBS) $(SDL) $^ -o $(BUILD)/$@ $(COMPCOMFLAG)$(subst $(BUILD)/,,$@).json

$(PLAYER): $(BUILD)/desktop_player/player.o $(BUILD)/include/note.o
	clang src/desktop_player/player_test.c $(CFLAGS) $(LIBS) $(SDL) $^ -o $(BUILD)/$@ $(COMPCOMFLAG)$(subst $(BUILD)/,,$@).json

$(BUILD)/%.o: src/%.c
	mkdir -p $(dir $@)
	mkdir -p $(dir build/compilecommands/$(subst $(BUILD)/,,$@))
	$(CC) $(CFLAGS) -c $< -o $@ $(COMPCOMFLAG)$(subst $(BUILD)/,,$@).json

build_arduino:
	# will overwrite compile_commands.json
	scripts/build_arduino.sh "src/ard_sound/" "build"

upload_arduino:
	scripts/upload_arduino.sh "build"

.PHONY: clean
clean:
	rm -r $(BUILD)