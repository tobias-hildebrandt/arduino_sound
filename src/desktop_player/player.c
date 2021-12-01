#include <SDL2/SDL.h>
#include <math.h>

#include <stdio.h>
#include <stdlib.h>

#include "../include/note.h"
#include "../include/note_math.h"

#define BUFFER_DURATION 1 // assume 60 bpm or 1 beat per second
#define FREQUENCY 48000 // samples per second
#define BUFFER_LEN (BUFFER_DURATION*FREQUENCY)
#define QUIET_FRACTION 1/10

void fill_buffer_with_note(struct Note* note, double volume);

short buffer[BUFFER_LEN]; // buffer that is used to fill audio and queue to SDL

// take value between 0-1
// multiply it by volume from 0 to 1
// normalize that to the maximum size of a short
short normalize(double value, double volume) {
	return (short)(value*volume*32567);
}

// TODO: make a cleaner version that either fades notes or tapers off at the end to prevent clicking sound
double sinewave(double freq, unsigned long timestamp) {
	return sin(timestamp * freq * M_PI * 2 / FREQUENCY);
}

double sawtooth(double freq, unsigned long timestamp) {
	return fmod(timestamp*freq/FREQUENCY, 1);
}

double square(double freq, unsigned long timestamp) {
	if (sinewave(freq, timestamp) > 0) {
		return 1;
	} else {
		return -1;
	}
}

struct SongData {
	struct Note** song;
	int current_note;
};

void sdl_play_song(struct Note** song, SDL_AudioDeviceID device) {
	for (int i = 0;; i++) {
		struct Note * cur_note = song[i];

		printf("node add is %p", cur_note);

		if (cur_note->pitch == end_note) {
			// end of song
			break;
		}
		
		fill_buffer_with_note(cur_note, 0.02); // very low volume
		SDL_QueueAudio(device, buffer, BUFFER_LEN);

		SDL_PauseAudioDevice(device, 0); // unpause device
		while (SDL_GetQueuedAudioSize(device) > (BUFFER_LEN / 2)) ; // wait until we need to refill it
	}

	while (SDL_GetQueuedAudioSize(device) > 0) ; // wait until no more audio left
}

int player_play_song(struct Note** song) {
	printf("player_play_song\n");

	if (song == NULL) {
		fprintf(stderr, "song is null!\n");
		exit(-1);
	}

	// init sdl
	if (SDL_Init(SDL_INIT_AUDIO) < 0) {
		printf("SDL failed to initialize : %s\n", SDL_GetError());
		return -1;
	}

	// sdl audio specification
	SDL_AudioSpec spec = {
		.freq = FREQUENCY, // samples per sec
		.format = AUDIO_S16SYS, // short, signed 16 bit integer
		.channels = 1, // mono, don't care for stereo
		.samples = 4096, // ??
		.callback = NULL, // use queue instead
		.userdata = NULL // use queue instead
	};

	// open default device and use our specification
	SDL_AudioDeviceID device = SDL_OpenAudioDevice(NULL, 0, &spec, NULL, 0);

	sdl_play_song(song, device);

	SDL_CloseAudioDevice(device);
	SDL_Quit();

	return 0;
}

void sdl_play_song2(struct Note* song, SDL_AudioDeviceID device) {
	for (int i = 0;; i++) {
		struct Note * cur_note = &song[i];

		if (cur_note->pitch == end_note) {
			// end of song
			break;
		}
		
		fill_buffer_with_note(cur_note, 0.02); // very low volume
		SDL_QueueAudio(device, buffer, BUFFER_LEN);

		SDL_PauseAudioDevice(device, 0); // unpause device
		while (SDL_GetQueuedAudioSize(device) > (BUFFER_LEN / 2)) ; // wait until we need to refill it
	}

	while (SDL_GetQueuedAudioSize(device) > 0) ; // wait until no more audio left
}

int player_play_song2(struct Note* song) {
	printf("player_play_song2\n");

	// init sdl
	if (SDL_Init(SDL_INIT_AUDIO) < 0) {
		printf("SDL failed to initialize : %s\n", SDL_GetError());
		return -1;
	}

	// sdl audio specification
	SDL_AudioSpec spec = {
		.freq = FREQUENCY, // samples per sec
		.format = AUDIO_S16SYS, // short, signed 16 bit integer
		.channels = 1, // mono, don't care for stereo
		.samples = 4096, // ??
		.callback = NULL, // use queue instead
		.userdata = NULL // use queue instead
	};

	// open default device and use our specification
	SDL_AudioDeviceID device = SDL_OpenAudioDevice(NULL, 0, &spec, NULL, 0);

	sdl_play_song2(song, device);

	SDL_CloseAudioDevice(device);
	SDL_Quit();

	return 0;
}

void fill_buffer_with_note(struct Note* note, double volume) {
	// if no note, set to 0
	if (note->pitch == no_note) {
		SDL_memset(buffer, 0, BUFFER_LEN);
		return;
	}

	// else calculate frequency and fill buffer with sound
	double freq = note_frequency(note->pitch);

	// cutoff between sound and silence
	int sound_count = (int)((double)BUFFER_LEN * (1.0-(double)QUIET_FRACTION));

	// from 0 to sound_count
	for (int i = 0; i < sound_count; i++) {
		buffer[i] = normalize(square(freq, i), volume);
	}

	// from sound_count to end
	// (void*) so we get byte array arthimetic
	SDL_memset((void*)buffer + sound_count, 0, BUFFER_LEN - sound_count);
}
