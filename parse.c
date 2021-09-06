#include "note.h"
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <stdbool.h>

#define BUFFER_SIZE 2048

const char note_chars[] = {
    'a','b','c','d','e','f','g',
    'A','B','C','D','E','F','G',
    0
};

struct State {
    struct Note* notes;
    int current_note ;
    bool started;
};

void parse_line(struct State* state, char *line);
bool char_is_note(char c);

int main(int argc, char** argv) {

    if (argc < 2) {
        printf("please give a filename\n");
        return 1;
    }

    char* filename = argv[1];

    struct State state = { 
        .notes = (struct Note*) malloc(sizeof(struct Note) * 64),
        .current_note = 0,
        .started = false
    };

    FILE *f = fopen("test.abc", "r");

    char buf[BUFFER_SIZE];
    while (fgets(buf, BUFFER_SIZE, f) != NULL) {
        // line includes final '\n'
        parse_line(&state, buf);
        printf("%s", buf);
    }
    printf("done!\n");
}

void parse_line(struct State* state, char *line) {
    // if we haven't yet reached the notes
    if (! state->started) {
        int len = strlen(line);
        if (len > 2 && line[0] == 'K' && line[1] == ':') {
            printf("found K line!: ");
            state->started = true;
        } else {
            printf("found useless line: ");
        }
        return;
    }

    for (int i = 0; ; i++) {
        char c = line[i];
        if (c == '\n') {
            printf("done parsing line: ");
            break;
        }
        if (char_is_note(c)) {
            printf("found note char %c\n", c);
        }
    }
}

bool char_is_note(char c) {
    int i = 0;
    char note_char = note_chars[i];
    while (note_char != 0){
        if (c == note_char) {
            return true;
        }
        i++;
        note_char = note_chars[i];
    }
    return false;
}