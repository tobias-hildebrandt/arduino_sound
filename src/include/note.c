#ifndef ARDSOUND_NOTE_C
#define ARDSOUND_NOTE_C

#include "note.h"

struct Note NOTE_C = { .pitch = 0, .length = 3};
struct Note NOTE_D = { .pitch = 2, .length = 3};
struct Note NOTE_E = { .pitch = 4, .length = 3};
struct Note NOTE_F = { .pitch = 5, .length = 3};
struct Note NOTE_G = { .pitch = 7, .length = 3};
struct Note NOTE_A = { .pitch = 9, .length = 3};
struct Note NOTE_B = { .pitch = 11, .length = 3};
struct Note NOTE_C_HIGH = { .pitch = 12, .length = 3};
struct Note NOTE_REST_FOURTH = { .pitch = no_note, .length = 3};
struct Note NOTE_REST_HALF = { .pitch = no_note, .length = 4};
struct Note NOTE_END = { .pitch = end_note, .length = 3};

struct Note* scale[] = {
    &NOTE_C, &NOTE_D, &NOTE_E, &NOTE_F, &NOTE_G, &NOTE_A, &NOTE_B, &NOTE_C_HIGH, &NOTE_END
};

struct Note* mary[] = {
    &NOTE_E, &NOTE_D, &NOTE_C, &NOTE_D, &NOTE_E, &NOTE_E, &NOTE_E, &NOTE_REST_FOURTH, 
    &NOTE_D, &NOTE_D, &NOTE_D, &NOTE_REST_FOURTH, &NOTE_E, &NOTE_G, &NOTE_G, &NOTE_REST_FOURTH,
    &NOTE_E, &NOTE_D, &NOTE_C, &NOTE_D, &NOTE_E, &NOTE_E, &NOTE_E, &NOTE_E, 
    &NOTE_D, &NOTE_D, &NOTE_E, &NOTE_D, &NOTE_C, &NOTE_END
};

#endif