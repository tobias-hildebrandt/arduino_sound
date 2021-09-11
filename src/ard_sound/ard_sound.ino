#include "clangd_arduino.h"
#include "note.h" // note that arduino-cli is fussy about this path, see /scripts/build_arduino.sh

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
const struct Note NOTE_REST_HALF = Note { .tone = no_note, .length = 4};
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
const struct Note* REST_HALF = &NOTE_REST_HALF;
const struct Note* END = &NOTE_END;

bool status_light = true;  // global variable in order to toggle internal led

// fn = f0 * (a)n
const double middle_C_frequency = 440;
const double math_A = pow(2, 1./12.); // 12th root of 2
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

unsigned long get_delay(char length, double tempo) {
    double measures = 1;
    // lookup table faster?
    // fraction = 1./32. * pow(2, length);
    switch (length) {
        case 0: measures = 1./32.; break;
        case 1: measures = 1./16.; break;
        case 2: measures = 1./8.; break;
        case 3: measures = 1./4.; break;
        case 4: measures = 1./2.; break;
        case 5: measures = 1; break;
        case 6: measures = 2; break;
        case 7: measures = 4; break;
        case 8: measures = 8; break;
    }

    // beats per minute, not second 
    // assume time signature denominator of 4
    // seconds to milliseconds
    return (unsigned long)((measures * 60 * 4 * 1000 / tempo)); 
}

void toggle_internal_led() {
    status_light = !status_light;
    digitalWrite(INTERNAL_PIN, status_light);
}

void play_note(struct Note *n, double tempo) {
    noTone(OUTPUT_PIN);
    
    // toggle_internal_led();

    if (n->tone != no_note) {
        tone(OUTPUT_PIN, note_frequency(n->tone));
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

    struct Note *c_notes[] = {
        &lower_c, 
        &low_c, 
        &mid_c, 
        &high_c, 
        &higher_c,
        &high_c, 
        &mid_c,
        &low_c, 
        &lower_c, 
        (struct Note*) END
    };

    play_song(c_notes, tempo);

}

/* min should not be below ~100*/
void test_raw_pitches(double min, double max) {
    double freq = min;
    noTone(OUTPUT_PIN);
    
    while(freq < max) {
        tone(OUTPUT_PIN, freq);
        delay(10);
        freq += pow(freq, 1./12.);
        // noTone(OUTPUT_PIN);
    }
}


void setup() {
    pinMode(OUTPUT_PIN, OUTPUT);
    pinMode(INTERNAL_PIN, OUTPUT);
}

void loop() {
    play_song((struct Note **) scale, 60);
    play_note((struct Note *) REST_HALF, 60);
    // play_song((struct Note **) mary, 120);
    // play_note((struct Note *) REST_HALF, 60);
    // test_c_notes(60);
    // play_note((struct Note *) REST_HALF, 60);
    test_raw_pitches(100, 2000);
    play_note((struct Note *) REST_HALF, 60);
}


