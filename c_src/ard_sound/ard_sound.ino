#include <Arduino.h>

#include <note.h>
#include <note_math.h>
#include "../../out/out.h"

#define OUTPUT_PIN 5 // digital pin 5
#define INTERNAL_PIN 13
#define DELAY 200

bool status_light = true;  // global variable in order to toggle internal led

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
    return (unsigned long)((measures * 60 * 1000 / tempo));
}

void toggle_internal_led() {
    status_light = !status_light;
    digitalWrite(INTERNAL_PIN, status_light);
}

void play_note(struct Note *n, double tempo) {
    noTone(OUTPUT_PIN);

    // TODO: debug clickiness, may need to be done via circuitry

    toggle_internal_led();

    if (n->pitch != no_note) {
        tone(OUTPUT_PIN, note_frequency(n->pitch));
    }
    delay(get_delay(n->length, tempo));
    noTone(OUTPUT_PIN);
}


void play_song(struct Note **song, double tempo) {
    for(int i = 0; ; i++) {
        if (song[i]->pitch != end_note) {
            play_note(song[i], tempo);
        } else {
            break;
        }
    }
}

void test_octaves(double tempo) {

    struct Note lower = { .pitch = -24, .length = 3};
    struct Note low = { .pitch = -12, .length = 3};
    struct Note mid = { .pitch = 0, .length = 3};
    struct Note high = { .pitch = 12, .length = 3};
    struct Note higher = { .pitch = 24, .length = 3};

    struct Note *octave_notes[] = {
        &lower,
        &low,
        &mid,
        &high,
        &higher,
        &high,
        &mid,
        &low,
        &lower,
        (struct Note*) &NOTE_END
    };

    play_song(octave_notes, tempo);

}

/* min should not be below ~100*/
void test_raw_pitches(double start, double max) {
    noTone(OUTPUT_PIN);

    while(start < max) {
        tone(OUTPUT_PIN, start);
        delay(100);
        toggle_internal_led();
        start += 100;
        // noTone(OUTPUT_PIN);
    }
}

void test_all_note_pitches(double tempo) {
    struct Note n = { .pitch = 0, .length = 3};

    // TODO: figure out why pitch -60 wont work, maybe it's too low for the buzzer?
    for(char i = -30; i < 20; i++) { // ~77 Hz to ~1400 Hz
        n.pitch = i;
        play_note(&n, tempo);
    }
}

void play_optimized_song(struct Note* lookup, short* song, double tempo) {
    for(short i = 0; ; i++) {
        short note_index = song[i];
        struct Note* current_note = &lookup[note_index];

        if (current_note->pitch == end_note) {
            break;
        }

        play_note(current_note, tempo);
    }
}

void simple_half_rest(double tempo) {
    play_note((struct Note *) &NOTE_REST_HALF, tempo);
}

void setup() {
    pinMode(OUTPUT_PIN, OUTPUT);
    pinMode(INTERNAL_PIN, OUTPUT);
}

void loop() {
    // TODO: split this into another file
    // play_song((struct Note **) scale, 60);
    // simple_half_rest(60);
    // play_song((struct Note **) mary, 120);
    // simple_half_rest(60);
    // test_octaves(60);
    // simple_half_rest(60);
    // test_raw_pitches(100, 2000);
    // simple_half_rest(60);
    // test_all_note_pitches(120);
    // simple_half_rest(60);
    play_optimized_song(note_lookup, song, 120.);
}


