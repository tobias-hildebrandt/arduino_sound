#include "scuffed_arduino.h"
#include <sys/types.h>
#include "note.h"

#define OUTPUT_PIN 7 // digital pin 7
#define INTERNAL_PIN 13
#define DELAY 200

const struct Note NOTE_C = Note { .tone = 0, .length = 3};
const struct Note NOTE_D = Note { .tone = 2, .length = 3};
const struct Note NOTE_E = Note { .tone = 4, .length = 3};
const struct Note NOTE_F = Note { .tone = 5, .length = 3};
const struct Note NOTE_G = Note { .tone = 7, .length = 3};
const struct Note NOTE_A = Note { .tone = 9, .length = 3};
const struct Note NOTE_B = Note { .tone = 11, .length = 3};
const struct Note NOTE_C_HIGH = Note { .tone = 12, .length = 3};
const struct Note NOTE_REST_FOURTH = Note { .tone = no_note, .length = 3};
const struct Note NOTE_END = Note { .tone = end_note, .length = 3};

const struct Note* C = &NOTE_C;
const struct Note* D = &NOTE_D;
const struct Note* E = &NOTE_E;
const struct Note* F = &NOTE_F;
const struct Note* G = &NOTE_G;
const struct Note* A = &NOTE_A;
const struct Note* B = &NOTE_B;
const struct Note* C_HIGH = &NOTE_C_HIGH;
const struct Note* REST_FOURTH = &NOTE_REST_FOURTH;
const struct Note* END = &NOTE_END;


// fn = f0 * (a)n
const double middle_C_frequency = 440;
const double math_A = pow(2, 1./12.);
double note_frequency(int half_steps_from_c) {
    unsigned int frequency = middle_C_frequency * pow(math_A, half_steps_from_c);
    return frequency;
}

const struct Note* scale[] = {
    C, D, E, F, G, A, B, C_HIGH, END
};

const struct Note* mary[] = {
    E, D, C, D, E, E, E, REST_FOURTH, 
    D, D, D, REST_FOURTH, E, G, G, REST_FOURTH,
    E, D, C, D, E, E, E, E, 
    D, D, E, D, C, END
};

unsigned long get_delay(byte length, double tempo) {
    double fraction = 1;
    // lookup table faster?
    // fraction = 1./32. * pow(2, length);
    switch (length) {
        case 0: fraction = 1./32.;
        case 1: fraction = 1./16.;
        case 2: fraction = 1./8.;
        case 3: fraction = 1./4.;
        case 4: fraction = 1./2.;
        case 5: fraction = 1;
        case 6: fraction = 2;
        case 7: fraction = 4;
        case 8: fraction = 8;
    }

    return (unsigned long)((fraction * 4 * 1000 / tempo)); // assume time signature denominator of 4
}

void play_note(struct Note *n, double tempo) {
    noTone(OUTPUT_PIN);
    if (n->tone != no_note) {
        // digitalWrite(INTERNAL_PIN, 1);
        tone(OUTPUT_PIN, note_frequency(n->tone));
    } else {
        // digitalWrite(INTERNAL_PIN, 0);
    }
    delay(get_delay(n->length, tempo));
    noTone(OUTPUT_PIN);
}


void play_song(struct Note **song, double tempo) {
    for(int i =0;; i++) {
        if (song[i]->tone != end_note) {
            play_note(song[i], tempo);
        } else {
            break;
        }
    }
}

void test_c_notes(double tempo) {

    struct Note lower_c =  Note { .tone = -24, .length = 3};
    struct Note low_c = Note { .tone = -12, .length = 3};
    struct Note mid_c = Note { .tone = 0, .length = 3};
    struct Note high_c = Note { .tone = 12, .length = 3};
    struct Note higher_c = Note { .tone = 24, .length = 3};

    struct Note *c_notes[5] = {
        &lower_c, &low_c, &mid_c, &high_c, &higher_c
    };

    for (int i = 0; i < 5; i++) {
        play_note(c_notes[i], tempo);
        play_note((struct Note *) REST_FOURTH, tempo);
    }
} 


void setup() {
    pinMode(OUTPUT_PIN, OUTPUT);
    pinMode(INTERNAL_PIN, OUTPUT);
}

void loop() {
    play_song((struct Note **) scale, 60);
    play_note((struct Note *) REST_FOURTH, 60);
    play_song((struct Note **) mary, 90);
    play_note((struct Note *) REST_FOURTH, 60);
    test_c_notes(60);
}


