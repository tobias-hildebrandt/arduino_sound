#include "../include/note.h"
#include <errno.h> // sudo ln -s /usr/include/asm-generic /usr/include/asm
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <stdbool.h>
#include <ctype.h>

#include "../desktop_player/player.h"

#define BUFFER_SIZE 2048
#define MAX_NOTES 256

// TODO: perfect hashing
// these would be better as hashmaps
const char pitch_chars[][2] = {
    {'C', 0}, {'D', 2}, {'E', 4}, {'F', 5}, {'G', 7}, {'A', 9}, {'B', 11},
    {'c', 12}, {'d', 14}, {'e', 16}, {'f', 17}, {'g', 19}, {'a', 21}, {'b', 23},
    {'z', no_note}, {'Z', no_note}, {'x', no_note},
    {0, 0}
};

const char accidental_chars[][2] = {
    {'^', 1}, {'=', 0}, {'_', -1},
    {0, 0}
};

const char octave_chars[][2] = {
    {'\'', 1}, {',', -1},
    {0, 0}
};

struct NoteBuilder {
    int accidentals; // how many half-steps off we should be
    int base_pitch;
    int octaves; // how many octaves to add up or down
    char length_string[8]; // should only have 8 characters of note length -- I think this is a reasonable assumption
    char complete_string[256];
    bool empty;
};


// TODO: use defines instead??
enum CharCategory {
    CATEGORY_NONE,
    CATEGORY_ACCIDENTAL,
    CATEGORY_PITCH,
    CATEGORY_OCTAVE,
    CATEGORY_LENGTH
};

enum BuilderStatus {
    BUILDER_SUCCESS,
    BUILDER_ERROR,
    BUILDER_DONE
};

//TODO: create necessary notes, 
//      then use pointers to make actual series of notes
//      to save ROM space
struct State {
    struct Note* song;
    int current_note;
    bool started;
};

void parse_line(struct State* state, char *line);
bool parse_char(char c, int* value, enum CharCategory* char_category);
enum BuilderStatus add_char_to_builder(struct NoteBuilder* current_builder, struct NoteBuilder* new_builder, char current_char, enum CharCategory *last_category);
const char* char_category_to_string(enum CharCategory cat);
bool build_note_from_builder(struct NoteBuilder* builder, struct Note* note);
void add_note_to_state(struct State* state, struct Note* note);
bool try_parse_using_data_array(char c, const char** data, int* value, enum CharCategory* char_category, enum CharCategory real_category);
void clear_builder(struct NoteBuilder* builder);
void build_add_manage_builders(struct NoteBuilder* current_builder, struct NoteBuilder* new_builder, struct State* state, struct Note* new_note);
unsigned char length_string_to_id(char* string);

int main(int argc, char** argv) {

    // printf("sizeof Note = %d\n", sizeof(struct Note));

    if (argc < 2) {
        printf("please give a filename\n");
        return 1;
    }

    char* filename = argv[1];

    FILE *file = fopen(filename, "r");

    if (file == NULL) {
        fprintf(stderr, "failed to open file \'%s\', errno = %d\n", filename, errno);
        exit(-1);
    }

    struct State state = { 
        .song = (struct Note*) malloc(sizeof(struct Note) * MAX_NOTES),
        .current_note = 0,
        .started = false
    };

    char buf[BUFFER_SIZE];
    
    printf("starting to parse file\n");
    while (fgets(buf, BUFFER_SIZE, file) != NULL) {
        // line includes final '\n'
        parse_line(&state, buf);
        printf("%s\n", buf); // buf always ends in newline, but we add a new one for more sanity
    }
    printf("done parsing\n\n");

    struct Note endnote = {
        .pitch = end_note,
        .length = 0
    };

    // terminate song
    state.song[state.current_note] = endnote;
    state.current_note += 1;

    printf("printing out notes: \n");
    for (int i=0;i<state.current_note;i++) {
        printf("note %d: t: %d, l: %d\n", i, state.song[i].pitch, state.song[i].length);
    }

    struct Song song = {
        .notes = state.song,
        .tempo = 360, // TODO parse this
    };

    player_play_song2(&song);

    free(state.song);
    return 0;
}

void parse_line(struct State* state, char *line) {
    // if we haven't yet reached the notes
    if (! state->started) {
        int len = strlen(line);

        // check for the K line
        if (len > 2 && line[0] == 'K' && line[1] == ':') {
            printf("found K line!: ");
            state->started = true;
        } else {
            printf("found useless line: ");
        }
        return;
    }

    struct NoteBuilder current_builder = {
        .accidentals = 0,
        .base_pitch = invalid_note,
        .octaves = 0,
        .length_string = "",
        .empty = true,
    };

    struct NoteBuilder new_builder = {
        .accidentals = 0,
        .base_pitch = invalid_note,
        .octaves = 0,
        .length_string = "",
        .empty = true,
    };

    enum CharCategory last_cat = CATEGORY_NONE;

    struct Note new_note = {
        .pitch = 0,
        .length = 0
    };

    // we need to parse notes now
    for (int i = 0; ; i++) {
        char current_char = line[i];
        // printf("found char %c (%d)\n", current_char, current_char);
        if (current_char == '\n') { // newline, stop parsing
            printf("\nend of line detected, building note\n");

            build_add_manage_builders(&current_builder, &new_builder, state, &new_note);
            
            printf("\ndone parsing line: ");
            break;
        } else if (current_char == '%') {
            printf("\ncomment detected, building note\n");

            build_add_manage_builders(&current_builder, &new_builder, state, &new_note);

            printf("\ndone parsing line (that ended with comment): ");
            break;
        } else {
            enum BuilderStatus status = add_char_to_builder(&current_builder, &new_builder, current_char, &last_cat);
            switch (status) {
                case BUILDER_ERROR:
                    // printf("UNRECOGNIZED char: \'%c\'\n", current_char);
                    break;
                case BUILDER_SUCCESS:
                    break;
                case BUILDER_DONE:
                    printf("\nnew note detected, building note...\n");
                    build_add_manage_builders(&current_builder, &new_builder, state, &new_note);
            }
        }
    }
}

enum BuilderStatus add_char_to_builder(struct NoteBuilder* current_builder, struct NoteBuilder* new_builder, char current_char, enum CharCategory *last_category) {
    int value = 0;
    enum CharCategory current_category = CATEGORY_NONE;
    bool success = parse_char(current_char, &value, &current_category);
    if (success) {

        printf("in %s: char \'%c\' has value: %d\n", char_category_to_string(current_category), current_char, value);
        
        struct NoteBuilder* target_builder = current_builder;

        enum BuilderStatus success_status = BUILDER_SUCCESS;
        
        if ( // anything besides pitch can have more than 1 character
            (current_category != CATEGORY_PITCH && current_category < *last_category) || 
            (current_category == CATEGORY_PITCH && current_category <= *last_category)
        ) { 
            // we have started a new note
            target_builder = new_builder;
            success_status = BUILDER_DONE;
            strncat(new_builder->complete_string, &current_char, 1);
        } else {
            strncat(current_builder->complete_string, &current_char, 1);
        }

        switch (current_category) {
            case CATEGORY_NONE:
                return BUILDER_ERROR;
            case CATEGORY_ACCIDENTAL:
                target_builder->accidentals += value;
                break;
            case CATEGORY_LENGTH:
                strncat(target_builder->length_string, &current_char, 1);
                break;
            case CATEGORY_OCTAVE:
                target_builder->octaves += value;
                break;
            case CATEGORY_PITCH:
                target_builder->base_pitch = value;
                break;
        }
        // set empty flag
        target_builder->empty = false;

        // update last category only after we know we added a good character
        *last_category = current_category;

        return success_status;
    } else {
        return BUILDER_ERROR;
    }
}

bool parse_char(char c, int* value, enum CharCategory* char_category) {

    if (try_parse_using_data_array(c, (const char**) accidental_chars, value, char_category, CATEGORY_ACCIDENTAL) 
        || try_parse_using_data_array(c, (const char**) pitch_chars, value, char_category, CATEGORY_PITCH) 
        || try_parse_using_data_array(c, (const char**) octave_chars, value, char_category, CATEGORY_OCTAVE)
    ) {
        return true;
    }
        
    // check for length
    if ((c >= '0' && c <= '9') || c == '/') {
        *value = (int) c;
        *char_category = CATEGORY_LENGTH;
        return true;
    }

    return false;

} 

bool try_parse_using_data_array(char c, const char** data, int* value, enum CharCategory* char_category, enum CharCategory real_category) {
    // printf("testing membership in %s\n", char_category_to_string(real_category));
    for (int i = 0; ; i++) {
        // printf("i is %d\n", i);
        const char* current_pair = (char*)data + 2*i; // pointer arithmetic, each sub-array is 2 elements long
        const char const_char = current_pair[0];
        const char const_value = current_pair[1];

        if (const_char == 0) { // reached end of category's array
            return false;
        }
        if (const_char == c ) { // we have a match
            // printf("found in %s\n", char_category_to_string(real_category));
            *char_category = (enum CharCategory) real_category;
            *value = const_value;
            return true;
        }
    }
    return false;
}

const char* char_category_to_string(enum CharCategory cat) {
    switch (cat) {
        case CATEGORY_NONE:
            return "NO CATEGORY";
        case CATEGORY_ACCIDENTAL:
            return "ACCIDENTAL";
        case CATEGORY_LENGTH:
            return "LENGTH";
        case CATEGORY_OCTAVE:
            return "OCTAVE";
        case CATEGORY_PITCH:
            return "PITCH";
    }
    return "ERROR";
}

bool build_note_from_builder(struct NoteBuilder* builder, struct Note* note) {

    if (builder->empty) {
        printf("builder is empty!\n");
        return false;
    }

    int length = 0;
    int pitch;

    pitch = builder->base_pitch;
    pitch += builder->accidentals; // accidentals are half-steps
    pitch += builder->octaves * 12; // octaves are 12 half steps

    length = length_string_to_id(builder->length_string);

    note->length = length;
    note->pitch = pitch;

    return true;
}

void add_note_to_state(struct State* state, struct Note* note) {
    state->song[state->current_note] = *note;
    state->current_note += 1;
}

void clear_builder(struct NoteBuilder* builder) {
    builder->accidentals = 0;
    builder->base_pitch = invalid_note;
    builder->octaves = 0;
    strcpy(builder->length_string, "");
    strcpy(builder->complete_string, "");
}

void build_add_manage_builders(struct NoteBuilder* current_builder, struct NoteBuilder* new_builder, struct State* state, struct Note* new_note) {
    // add the note from the builder to the state
    bool should_add = build_note_from_builder(current_builder, new_note);
    if (should_add) {
        printf("built note from string \"%s\": pitch %d, length %d\n", current_builder->complete_string, new_note->pitch, new_note->length);
        add_note_to_state(state, new_note);
    } else {
        printf("not adding because empty builder\n");
    }
    
    // copy new builder to the current one
    *current_builder = *new_builder;
    printf("\nnew builder starts with \"%s\"\n", current_builder->complete_string);
    // clear the now garbage builder
    clear_builder(new_builder);
}

unsigned char length_string_to_id(char* string) {
    string[7] = 0; // make sure string is a string
    int num_chars = strnlen(string, 8);
    bool divided = false;
    int length = -1;
    if (num_chars == 0) {
        return 5; // whole note
    }
    if (string[0] == '/') { // shorter than base
        if (num_chars == 1) { // "/" is "/2"
            return 4; // half note
        }
        divided = true;
        length = atoi(string+1); // ignore first char which is '/'
    } else { // longer than base
        length = atoi(string);
    }

    // TODO: maybe make this a separate function or lookup table
    // TODO: don't assume 4/4 time
    if (divided) {
        switch (length) {
            case 32: return 0;
            case 16: return 1;
            case 8: return 2;
            case 4: return 3;
            case 2: return 4;
            case 1: return 5;
            default:
                fprintf(stderr, "length of note has invalid number after /\n");
                return -1;
        }
    } else {
        switch (length) {
            case 1: return 5;
            case 2: return 6;
            case 4: return 7;
            case 8: return 8;
            case 16: return 9;
            default:
                fprintf(stderr, "length of note has invalid number\n");
                return -1;
        }
        
    }
}