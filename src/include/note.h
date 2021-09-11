#define no_note 31
#define end_note -31
#define invalid_note -9999
struct Note {
    signed char tone:6; // half steps away from middle C, -31 to +31, see special cases
    char length:4; // 0 = 1/32, 1 = 1/16, 2 = 1/8, 3 = 1/4, 4 = 1/2, 5 = 1, 6 = 2, 7 = 4, 8 = 8
};