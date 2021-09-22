#define no_note 127
#define end_note -127
#define invalid_note -9999
struct Note {
    signed char pitch:8; // half steps away from middle C, -127 to +127, see special cases
    char length:4; // 0 = 1/32, 1 = 1/16, 2 = 1/8, 3 = 1/4, 4 = 1/2, 5 = 1, 6 = 2, 7 = 4, 8 = 8
};