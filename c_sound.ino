#include "scuffed_arduino.h"
#include <sys/types.h>

#define OUTPUT_PIN 7 // digital pin 7
#define INTERNAL_PIN 13
#define DELAY 200

#define byte char

#define lowA -3
#define lowB -1
#define C 0 
#define D 2
#define E 4
#define F 5
#define G 7
#define A 9
#define B 11
#define highC 12
#define N -999
#define END_NOTE -9999

// fn = f0 * (a)n
const double middle_C = 440;
const double math_A = pow(2, 1./12.);
double note(int half_steps_from_c) {
    unsigned int frequency = middle_C * pow(math_A, half_steps_from_c);
    return min(frequency, 1000);
}

const unsigned int scale[] = {
    C, D, E, F, G, A, B, highC, END_NOTE
};

const unsigned int mary[] = {
    E, D, C, D, E, E, E, N, 
    D, D, D, N, E, G, G, N,
    E, D, C, D, E, E, E, E, 
    D, D, E, D, C, END_NOTE
};

void play_song(unsigned int *song) {
    for(int i =0;; i++) {
        // stop playing the note
        noTone(OUTPUT_PIN);

        // write debug pin
        digitalWrite(INTERNAL_PIN, i%2);

        // if we need to end
        if (song[i] == END_NOTE) {
            break;
        } 
        // else if we actually have a note to play
        else if (song[i] != N) {
            tone(OUTPUT_PIN, note(song[i]));
        }

        // delay 
        delay(DELAY);
    }
}


// const unsigned int tones[] = {
//     262,294,330,349,392,440,494,524
// };


#define no_note 63
struct Note {
    byte tone; // 0-11 octave 0, 12-23 octave 1, 24-35 octave 2, 36-47 octave 3, 48-59 octave 4, 63 = no_note
    byte length:4; // 0 = 1/32, 1 = 1/16, 2 = 1/8, 3 = 1/4, 4 = 1/2, 5 = 1, 6 = 2, 7 = 4, 8 = 8
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

    return (unsigned long)((fraction * tempo));
}

void play_note(struct Note *n, double tempo) {
    noTone(OUTPUT_PIN);
    if (n->tone != no_note) {
        // digitalWrite(INTERNAL_PIN, 1);
        tone(OUTPUT_PIN, note(n->tone - 24));
    } else {
        // digitalWrite(INTERNAL_PIN, 0);
    }
    delay(get_delay(n->length, tempo));
}

void aaaa() {
    double tempo = 60;

    struct Note mid_c;
    mid_c.tone = 24;
    mid_c.length = 3;

    struct Note rest;
    rest.tone = no_note;
    rest.length = 3;

    struct Note high_c;
    high_c.tone = 36;
    high_c.length = 3;

    play_note(&mid_c, tempo);
    play_note(&rest, tempo);
    play_note(&high_c, tempo);
    play_note(&rest, tempo);
} 


void setup() {
    pinMode(OUTPUT_PIN, OUTPUT);
    pinMode(INTERNAL_PIN, OUTPUT);
}

void loop() {
    // play_song((unsigned int *) scale);
    // delay(DELAY * 4);
    // play_song((unsigned int *) mary);
    aaaa();
    // delay(DELAY * 4);
}


