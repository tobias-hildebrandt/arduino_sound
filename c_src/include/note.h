#ifndef ARDSOUND_NOTE_H
#define ARDSOUND_NOTE_H

#define no_note 127
#define end_note -127
#define invalid_note -9999

#define DEFAULT_LENGTH 3

// TODO: move to implementation?
struct Note {
    signed char pitch:8; // half steps away from middle C, -127 to +127, see special cases
    unsigned char length:4; // length ID, see note.c and parse.c for details
};

struct Song {
    struct Note* notes; // array of notes
    int num_notes;
    int tempo; // beats per minute
};

unsigned int note_to_int(struct Note* note);

extern struct Note NOTE_C;
extern struct Note NOTE_D;
extern struct Note NOTE_E;
extern struct Note NOTE_F;
extern struct Note NOTE_G;
extern struct Note NOTE_A;
extern struct Note NOTE_B;
extern struct Note NOTE_C_HIGH;
extern struct Note NOTE_REST_FOURTH;
extern struct Note NOTE_REST_HALF;
extern struct Note NOTE_END;

// TODO: forward declare without sizes??
extern struct Note* scale[9];

extern struct Note* mary[30];

double length_id_to_fraction(unsigned char id);

void print_note(struct Note* note);

#endif
