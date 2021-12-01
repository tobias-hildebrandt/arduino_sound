#ifndef ARDSOUND_NOTE_H
#define ARDSOUND_NOTE_H

#define no_note 127
#define end_note -127
#define invalid_note -9999

// TODO: move to implementation?
struct Note {
    signed char pitch:8; // half steps away from middle C, -127 to +127, see special cases
    char length:4; // 0 = 1/32, 1 = 1/16, 2 = 1/8, 3 = 1/4, 4 = 1/2, 5 = 1, 6 = 2, 7 = 4, 8 = 8
};

// TODO: extern?
struct Note NOTE_C;
struct Note NOTE_D;
struct Note NOTE_E;
struct Note NOTE_F;
struct Note NOTE_G;
struct Note NOTE_A;
struct Note NOTE_B;
struct Note NOTE_C_HIGH;
struct Note NOTE_REST_FOURTH;
struct Note NOTE_REST_HALF;
struct Note NOTE_END;

// TODO: forward declare without sizes??
struct Note* scale[9];

struct Note* mary[30];

#endif