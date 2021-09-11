#include "../include/note.h"
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <stdbool.h>

#define BUFFER_SIZE 2048
#define MAX_NOTES 256

const char note_chars[][2] = {
    {'A', -3}, {'B', -1}, {'C', 0}, {'D', 2}, {'E', 4}, {'F', 5}, {'G', 7},
    {'a', 9}, {'b', 11},{'c', 12},{'d', 14},{'e', 16},{'f', 17},{'g', 19},
    {0, 0}
};

//TODO: create necessary notes, 
//      then use pointers to make actual series of notes
//      to save ROM space
struct State {
    struct Note* notes; 
    int current_note;
    bool started;
};

void parse_line(struct State* state, char *line);
bool char_is_note(char c, int *tone);

int main(int argc, char** argv) {

    printf("sizeof Note = %d\n", sizeof(struct Note));

    if (argc < 2) {
        printf("please give a filename\n");
        return 1;
    }

    char* filename = argv[1];

    struct State state = { 
        .notes = (struct Note*) malloc(sizeof(struct Note) * MAX_NOTES),
        .current_note = 0,
        .started = false
    };

    FILE *p_file = fopen(filename, "r");

    char buf[BUFFER_SIZE];
    while (fgets(buf, BUFFER_SIZE, p_file) != NULL) {
        // line includes final '\n'
        parse_line(&state, buf);
        printf("%s\n", buf);
    }
    printf("done parsing\n");
    printf("printing out notes: \n");
    for (int i=0;i<state.current_note;i++) {
        printf("note %d: t: %d, l: %d\n", i, state.notes[i].tone, state.notes[i].length);
    }
}

void parse_line(struct State* p_state, char *line) {
    // if we haven't yet reached the notes
    if (! p_state->started) {
        int len = strlen(line);

        // check for the K line
        if (len > 2 && line[0] == 'K' && line[1] == ':') {
            printf("found K line!: ");
            p_state->started = true;
        } else {
            printf("found useless line: ");
        }
        return;
    }

    int tone = invalid_note;

    // we need to parse notes now
    for (int i = 0; ; i++) {
        char c = line[i];
        if (c == '\n') { // newline, stop parsing
            printf("done parsing line: ");
            break;
        }
        if (char_is_note(c, &tone)) {
            printf("found note char %c\n", c);
            if (tone == invalid_note) {
                printf("tone is invalid_note, exiting");
                exit(-1);
            }
            printf("adding note of tone: %d\n", tone);
            
            // TODO: implement length
            p_state->notes[p_state->current_note].length = 5;
            p_state->notes[p_state->current_note].tone = tone;
            p_state->current_note++;
        }
    }
}

bool char_is_note(char c, int *p_tone) {
    for (int i=0; ; i++) {
        char* current_check = (char*) note_chars[i];
        // printf("current check: {%c, %d} ", current_check[0], current_check[1]);
        if (current_check[0] == 0) { // reached end of array
            *p_tone = invalid_note;
            return false;
        }
        if (current_check[0] == c ) { // we have a match
            *p_tone = current_check[1];
            return true;
        }
    }
    return false;
}