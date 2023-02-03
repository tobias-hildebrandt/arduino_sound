# An Arduino Sound Project

This is a personal project exploring the concepts of simple digital sound/music,
code generation, and microcontrollers.
The main goal of this project was to have an Arduino Uno play music
written initially contained in an [`.abc`](https://abcnotation.com/) file.
I originally chose to write it in C, but I've re-written the entire stack in
Rust.

### Core functionality

- [x] sequences of sounds can be played via passive buzzer on the Arduino
- [ ] high-precision time keeping
- [ ] `.abc` files can be parsed using a host computer
  - [x] pitch parsing
  - [x] length parsing
  - [x] switch to ~~regex?~~ parser expression grammar
  - [ ] test coverage including sample `.abc` files
  - [ ] ABC time signature
  - [ ] ABC key signature
- [x] C header files (`.h`) can be generated on the host computer
- [x] Arduino C/C++ program can include the generated header file and
  play its contents
- [ ] user-friendly command line interface
  - [ ] one command to parse a file, generate code, then build and upload the
    Arduino program
- [x] host computer program can play and test parsing/generation without the
  need of an Arduino

### Future Possible Expansions

- [ ] parse MusicXML files
- [ ] allow for multiple buzzer to play harmonies/chords
- [x] re-write in Rust 🦀
- [ ] Windows support for build and upload scripts

## Getting Started

Note this has only been tested on Linux (Debian 10 and 11), though it should
work on any Unix-like system (BSD, MacOS, Solaris, etc.).
You might need to change the upload script if your Arduino serial port is not
connected to `/dev/ttyACM*`.

### Rust Setup
- Ensure the Rust toolchain is installed (e.g. using
  [rustup](https://rustup.rs/)).
- (Optional) Set up [rust-analyzer](https://rust-analyzer.github.io/) for your
  editor/IDE. If you do, make sure to add `ard-r-sound-embedded`
  (the Arduino executable crate) to `rust-analyzer.files.excludeDirs`.
  This is necessary because the crate must be on `nightly` and has a
  specific forced build target.
  This also means that if you want to use `rust-analyzer` with this crate,
  you need to open another instance of your editor/IDE from _inside_ of
  its directory.

### C Setup
- (Optional) Set up [clangd](https://clangd.llvm.org/) for your editor/IDE.
  I use [vscode-clangd](https://github.com/clangd/vscode-clangd) with
  [VSCodium](https://vscodium.com/).
  The Arduino build process will still use  `avr-gcc` from `arduino-cli`
  (see next step), `clangd` is just for language support in the editor.
  In order for the build scripts to generate valid and complete
  `compile_commands.json` compilation databases (using `-MJ`),
  `clang` is required.
- (for Arduino support) Install
  [arduino-cli](https://arduino.github.io/arduino-cli/0.19/installation/).
  This is necessary for the build scripts.
  Personally, I downloaded the archive, extracted it to a directory, and
  added `arduino-cli` to my `PATH` environment variable.
  Alternatively, you could use the
  [Arduino IDE](https://www.arduino.cc/en/software),
  [Arduino-Makefile](https://github.com/sudar/Arduino-Makefile),
  or something else.
  - Install the necessary files for your board. For my Arduino Uno, I had
    to run:

    `arduino-cli config init`

    `arduino-cli core update-index`

    `arduino-cli core install arduino:avr`
- (for desktop player) Install `clang` and the SDL2 library.
  On Debian-based systems, this is done via
  `sudo apt-get install clang libsdl2-dev`.

## Usage of Rust Implementation

### Desktop tool
Build via `cargo build`
  - add `--release` if you want an optimized build
  - the `ard-r-sound` binary will be in `target/debug` or `target/release/`

Usage:
- `ard-r-sound <input_file.abc> [-f <output format>] [-o <output_file_path>]`
- `-f` options (all require a `-o` except `play`)
  - `raw` = raw PCM audio file
  - `wav` = WAV audio file
    (via [wav](https://crates.io/crates/wav))
  - `header` = generate a C header for use with the C implementation
    - Note that the C implementation looks for `out/out.h`
  - `play` = play audio through computer speakers (via
    [`cpal`](https://github.com/RustAudio/cpal))

### Arduino
The `ard-r-sound-embedded` crate builds into an Arduino executable.

Using `arduino_hal`, it can be built and uploaded in a single command
(assuming you've done `cd ard-r-sound-embedded/` first!):
- `cargo run -- -P <device_file>` (probably /dev/ttyACM0)

The `ard_r_sound_macros::static_from_file!{variable_name, file_path}`
procedural macro parses and optimizes an abc file.
- `variable_name` must be a valid Rust identifier.
- `file_path` is a file path -- not a `String` or `&str` or `Path`
  - This is implemented by concatenating the rest of the syntax tokens passed
    to the macro.

For example,
`static_from_file!{SONG, ../misc/example-abcs/mary.abc}`
expands to something like:
```rs
static SONG: ard_r_sound_base::OptimizedStatic<...> =
    ard_r_sound_base::OptimizedStatic<...> {
      // unique notes
      uniques: [...],
      // the song itself, stored as indexes of the uniques array
      list: [...],
    };
```
The `OptimizedStatic` struct is const-generic over the amount of unique
notes in the song and total number of notes in the song.

## Usage of C Implementation

**The C parser, code-generator, and SDL desktop player
are incomplete and no longer updated.
Use the Rust implementation for a more complete feature set.**

### `make <file.abc>`

Run the parser on a file.
This generates a C header file (`out/out.h`) containing an optimized
representation of the song.

### `make build_arduino`

Build the program for an Arduino Uno.
Uses the song-header in `out/out.h` (which can be generated by either the
Rust or C implementation).

### `make upload_arduino`

Attempt to upload the built program to **every device in /dev/ttyACM\***.
This is done because (on my machine, at least) power-cycling an Arduino may
change its device file between `ttyACM0` and `ttyACM1`.

### `make desktop`

Build the parser and desktop player.
This will create the executable `build/desktop/parse`.

Building the player requires SDL2.


## Architecture and Design

### Wiring Schematic
The Arduino should wired up similarly to the following diagram:

![Wiring Diagram](/misc/wiring/wiring_diagram.png)

(diagram made with [Fritzing](https://fritzing.org/) and
[GIMP](https://www.gimp.org/))

The toggle switch is there in order to disable the sound output without
unplugging the Arduino.
Additionally, the potentiometer is put in-line to reduce the volume of
the buzzer.
These are useful for testing but not necessary.
A resistor could be used in place of the potentiometer if a constant volume
reduction is needed.

The specific digital pin that you use isn't important,
but it is currently hardcoded to be ***pin 5***.

### Software Design

The `abc` file is parsed into an internal representation, then converted
into a more space-efficient format.

The main goal of this approach is to reduce executable size.
Via [the Arduino website](https://www.arduino.cc/en/pmwiki.php?n=Tutorial/Memory):

> The ATmega328 chip found on the Uno has the following amounts of memory:
>
> `Flash  32k bytes (of which .5k is used for the bootloader)`
>
> `SRAM   2k bytes`
>
> `EEPROM 1k byte`

By pre-processing the song, we are able to de-duplicate the notes and
save space.
Thus, the song in Arduino memory is not an array of "full-fat" notes,
but two arrays: one that contains unique notes and another
that contains indexes/references to those unique notes.
Since this is all done before the program is actually compiled,
this data can reside in read-only memory instead of RAM.
The actual compression/efficiency ratio of this technique depends on the
number of repeated notes of a specific song.

## License

Unless otherwise noted, all files in this repository are released under the
terms of the GNU General Public License version 3 or, at your choice,
any later version. (GPLv3+). See [COPYING](COPYING) for more details.

## Additional Reading

- `.abc` notation:
  [ABC Notation Homepage](https://abcnotation.com/),
  [ABC examples](https://abcnotation.com/examples),
  [the ABC Standard](https://abcnotation.com/wiki/abc:standard:v2.1),
  [the ABC Plus Project](http://abcplus.sourceforge.net/)
- Formula for note frequency:
  [One explanation](https://pages.mtu.edu/~suits/NoteFreqCalcs.html),
  [another explanation](https://en.wikipedia.org/wiki/Piano_key_frequencies)
- Making tones in SDL:
  [SDL2 tone generator](https://gist.github.com/jacobsebek/10867cb10cdfccf1d6cfdd24fa23ee96)

