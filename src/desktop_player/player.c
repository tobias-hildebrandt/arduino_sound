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
	// contains exactly enough samples for 1 beat
	// 60 because beats per minute must be translate to seconds
	int buffer_size = sizeof(short) * FREQUENCY * 60 / (song->tempo);

	// buffer on stack is fine because its lifetime is limited to this function
	short* buffer[buffer_size];
	
	printf("buffer size: %d kB\n", buffer_size / 1024);

	int beats_left_in_note = -1;
	struct Note * cur_note = song->notes;

	while (cur_note->pitch != end_note) {
		// printf("queueing note: p%d l%d\n", cur_note->pitch, cur_note->length);
		// how many beats left we need to write
		beats_left_in_note = length_id_to_fraction(cur_note->length); 

		// always write something, even if less than one whole beat,
		// <1 beat is handled
		if (beats_left_in_note == 0) {
			beats_left_in_note = 1;
		}

		// while we haven't written all the beats of this note
		while (beats_left_in_note > 0) {
			// fill the buffer (very low volume)
			int fill_length = fill_buffer_with_note(cur_note, song->tempo, 0.02, (short*) buffer, buffer_size); 

			// tell SDL to copy our buffer to its internal queue
			SDL_QueueAudio(device, buffer, fill_length);

			// unpause device
			SDL_PauseAudioDevice(device, 0); 

			// we just wrote a note (or less if needed)
			beats_left_in_note--; 

			// block until we need to refill it so we don't waste memory inside SDL internal buffer 
			// we still have to leave us some time to queue up the next note
			while (SDL_GetQueuedAudioSize(device) > (buffer_size / 2)); 
		}

		// we have written out the entire note, so go to the next
		cur_note++;
	}

	// wait until no more audio left
	while (SDL_GetQueuedAudioSize(device) > 0) ; 
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
	// exactly one beat
	int one_beat = (int) (FREQUENCY * 60.0 / tempo);
	// printf("one beat is %d samples\n", one_beat);
	int to_fill = (int)((double) one_beat * length_id_to_fraction(note->length));
	bool play_some_silence = true; // assume we are playing less than a full beat

	if (to_fill > buffer_size) { // need to write more than size of buffer
		to_fill = buffer_size; // write as much as we can, we'll do more next time
		play_some_silence = false; // don't stop playing the sound for a (hopefully) seamless transition
	}

	// cutoff between sound and silence
	int sound_count;
	if (play_some_silence) {
		sound_count = (int) ((double) to_fill * (1.0 - (double) QUIET_FRACTION));
	} else {
		sound_count = to_fill;
	}
	 
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
	if (play_some_silence) {
		SDL_memset((void*)buffer + sound_count, 0, to_fill - sound_count);
	}

	return to_fill;
}
