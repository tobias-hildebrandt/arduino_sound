#include <SDL2/SDL.h>
#include <math.h>

#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

#include "../include/note.h"
#include "../include/note_math.h"

#define MAX_SECONDS 16 
#define FREQUENCY 48000 // samples per second
#define BUFFER_LEN (MAX_SECONDS*FREQUENCY)
#define QUIET_FRACTION 1/10

struct SongData {
	struct Note** song;
	int current_note;
};

int fill_buffer_with_note(struct Note* note, int tempo, double volume, short* buffer, int buffer_size);

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

// void sdl_play_song(struct Note** song, SDL_AudioDeviceID device) {
// 	for (int i = 0;; i++) {
// 		struct Note * cur_note = song[i];

// 		if (cur_note->pitch == end_note) {
// 			// end of song
// 			break;
// 		}
		
// 		fill_buffer_with_note(cur_note, 0.02); // very low volume
// 		SDL_QueueAudio(device, buffer, BUFFER_LEN);

// 		SDL_PauseAudioDevice(device, 0); // unpause device
// 		while (SDL_GetQueuedAudioSize(device) > (BUFFER_LEN / 2)) ; // wait until we need to refill it
// 	}

// 	while (SDL_GetQueuedAudioSize(device) > 0) ; // wait until no more audio left
// }

// int player_play_song(struct Note** song) {
// 	printf("player_play_song\n");

// 	if (song == NULL) {
// 		fprintf(stderr, "song is null!\n");
// 		exit(-1);
// 	}

// 	// init sdl
// 	if (SDL_Init(SDL_INIT_AUDIO) < 0) {
// 		printf("SDL failed to initialize : %s\n", SDL_GetError());
// 		return -1;
// 	}

// 	// sdl audio specification
// 	SDL_AudioSpec spec = {
// 		.freq = FREQUENCY, // samples per sec
// 		.format = AUDIO_S16SYS, // short, signed 16 bit integer
// 		.channels = 1, // mono, don't care for stereo
// 		.samples = 4096, // ??
// 		.callback = NULL, // use queue instead
// 		.userdata = NULL // use queue instead
// 	};

// 	// open default device and use our specification
// 	SDL_AudioDeviceID device = SDL_OpenAudioDevice(NULL, 0, &spec, NULL, 0);

// 	sdl_play_song(song, device);

// 	SDL_CloseAudioDevice(device);
// 	SDL_Quit();

// 	return 0;
// }

void sdl_play_song2(struct Song* song, SDL_AudioDeviceID device) {
	int buffer_size = sizeof(short) * FREQUENCY * (song->tempo / 60); // 1 beat

	/*	
		BPM beats / minute
		60 minutes / second
		FREQ samples / second
		????
		TODO: get the math right
	*/
	short* buffer = malloc(buffer_size); // TODO: stack allocation?
	
	printf("buffer size: %d kB\n", buffer_size / 1024);

	int beats_left_in_note = -1; // trigger on first iteration
	struct Note * cur_note = song->notes;
	while (1) {
		if (beats_left_in_note <= 0) { // we have queued up all of the previous note
			cur_note += 1; // move to the next note
			beats_left_in_note = length_id_to_fraction(cur_note->length); // update beats remaining
		}
		
		if (cur_note->pitch == end_note) {
			// end of song
			break;
		}
		
		// very low volume
		int fill_length = fill_buffer_with_note(cur_note, song->tempo, 0.02, buffer, buffer_size); 

		SDL_QueueAudio(device, buffer, fill_length);

		SDL_PauseAudioDevice(device, 0); // unpause device

		beats_left_in_note--; // we just wrote a note or less if needed

		// this will give us half a beat to queue up the next note
		while (SDL_GetQueuedAudioSize(device) > (buffer_size / 2)); // block until we need to refill it
	}

	while (SDL_GetQueuedAudioSize(device) > 0) ; // wait until no more audio left

	free(buffer);
}

int player_play_song2(struct Song* song) {
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

int fill_buffer_with_note(struct Note* note, int tempo, double volume, short* buffer, int buffer_size) {
	double note_beats = length_id_to_fraction(note->length);
	int to_fill = (int) (FREQUENCY * (tempo / 60.0) * note_beats);
	if (to_fill > buffer_size) { // need to write more than size of buffer
		to_fill = buffer_size; // write as much as we can, we'll do more next time
	}

	// cutoff between sound and silence
	int sound_count = (int) ((double) to_fill * (1.0 - (double) QUIET_FRACTION));

	// from 0 to sound_count
	
	if (note->pitch != no_note) { // if not a rest note
		// calculate frequency and fill buffer with wave
		double freq = note_frequency(note->pitch);
		for (int i = 0; i < sound_count; i++) {
			buffer[i] = normalize(square(freq, i), volume);
		}
	} else { // rest note, just copy in 0s
		SDL_memset(buffer, 0, to_fill);
	}

	// from sound_count to end
	// (void*) so we get byte array arthimetic
	SDL_memset((void*)buffer + sound_count, 0, to_fill - sound_count);

	return to_fill;
}
