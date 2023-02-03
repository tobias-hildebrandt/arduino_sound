#ifndef ARDSOUND_NOTE_C
#define ARDSOUND_NOTE_C

#include "note.h"

struct Note NOTE_C = { .pitch = 0, .length = DEFAULT_LENGTH};
struct Note NOTE_D = { .pitch = 2, .length = DEFAULT_LENGTH};
struct Note NOTE_E = { .pitch = 4, .length = DEFAULT_LENGTH};
struct Note NOTE_F = { .pitch = 5, .length = DEFAULT_LENGTH};
struct Note NOTE_G = { .pitch = 7, .length = DEFAULT_LENGTH};
struct Note NOTE_A = { .pitch = 9, .length = DEFAULT_LENGTH};
struct Note NOTE_B = { .pitch = 11, .length = DEFAULT_LENGTH};
struct Note NOTE_C_HIGH = { .pitch = 12, .length = DEFAULT_LENGTH};
struct Note NOTE_REST_FOURTH = { .pitch = no_note, .length = 3};
struct Note NOTE_REST_HALF = { .pitch = no_note, .length = 4};
struct Note NOTE_END = { .pitch = end_note, .length = DEFAULT_LENGTH};

struct Note* scale[] = {
    &NOTE_C, &NOTE_D, &NOTE_E, &NOTE_F, &NOTE_G, &NOTE_A, &NOTE_B, &NOTE_C_HIGH, &NOTE_END
};

struct Note* mary[] = {
    &NOTE_E, &NOTE_D, &NOTE_C, &NOTE_D, &NOTE_E, &NOTE_E, &NOTE_E, &NOTE_REST_FOURTH, 
    &NOTE_D, &NOTE_D, &NOTE_D, &NOTE_REST_FOURTH, &NOTE_E, &NOTE_G, &NOTE_G, &NOTE_REST_FOURTH,
    &NOTE_E, &NOTE_D, &NOTE_C, &NOTE_D, &NOTE_E, &NOTE_E, &NOTE_E, &NOTE_E, 
    &NOTE_D, &NOTE_D, &NOTE_E, &NOTE_D, &NOTE_C, &NOTE_END
};

double length_id_to_fraction(unsigned char id) {
    // how many measures is the note? not beats
    // assume 4/4 for now
    // TODO: take time signature for input and calculate it
    switch (id) {
        case 0: return 1./32.;
        case 1: return 1./16.;
        case 2: return 1./8.;
        case 3: return 1./4.;
        case 4: return 1./2.;
        case 5: return 1;
        case 6: return 2;
        case 7: return 4;
        case 8: return 8;
        case 9: return 16;
        default: return -1;
    }
}

unsigned int note_to_int(struct Note* note) {
    unsigned int i = 0;
    i |= (unsigned int) (note->length << 8);
    i |= (unsigned int) (note->pitch);
    return i;
}

#endif