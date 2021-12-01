#ifndef ARDSOUND_PLAYER
#define ARDSOUND_PLAYER
// forward declarations
struct Note;
typedef struct SDL_AudioDeviceID SDL_AudioDeviceID;

void player_play_song(struct Note** song);
void player_play_song2(struct Note* song);

#endif