#include "player.h"
#include "../include/note.h"

int main(int argc, char* argv[]) {

	struct Note* rest_song[] = {
		&NOTE_REST_FOURTH, &NOTE_END
	};

	// TODO: fix null deference or something
	player_play_song(mary);
	player_play_song(rest_song);
	player_play_song(scale);

	return 0;
}