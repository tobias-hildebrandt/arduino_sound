#ifndef ARDSOUND_NOTE_MATH_H
#define ARDSOUND_NOTE_MATH_H

// require power function
extern double pow(double, double);

// fn = f0 * (a)^n
double MIDDLE_A_FREQ = 440;
double MATH_A = 1.059463094359; // 12th root of 2 or 2^(1/12)
double note_frequency(int half_steps_from_c) {
    double frequency = MIDDLE_A_FREQ * pow(MATH_A, half_steps_from_c);
    return frequency;
}

#endif