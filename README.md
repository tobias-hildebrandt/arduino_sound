# An Arduino Sound Project

This is a personal project exploring the concepts of digital sound/music, code generation, and microcontrollers. The main goal is to create software that will enable an Arduino Uno to play music initially contained in an [`.abc`](https://abcnotation.com/) file. I have chosen to write it in C in order to further my experience in the language.

## Current Progress

This project is still in its early stages, so please expect bugs. I do not recommend using this in production.

### Core functionality 

- [x] sequences of sounds can be played via passive buzzer on the Arduino
- [ ] have the Arduino keep exact time instead of waiting for instructions between notes (probably only gain a couple of milliseconds at best)
- [ ] `.abc` files can be parsed using a host computer
- [ ] C header files (`.h`) are generated on the host computer
- [ ] user-friendly command line interface
- [ ] host computer program to play and test headers without the need of an Arduino

### Future Possible Expansions

- [ ] parse MusicXML files
- [ ] allow for multiple buzzer to play harmonies
- [ ] re-write in Rust 🦀 (or another Arduino-compatible language)
- [ ] Windows support for build and upload scripts

## Getting Started

Note this has only been tested on Linux (Debian 10), though it should work on any Unix-like system (BSD, MacOS, Solaris, etc.). You might need to change the upload script if your Arduino is not at `/dev/ttyACM*`.

- Clone this repository (`git clone`)
- (Optional) Set up [clangd](https://clangd.llvm.org/) for your editor/IDE of choice. This helps with code completion, errors, etc. I use [vscode-clangd](https://github.com/clangd/vscode-clangd) with [VSCodium](https://vscodium.com/) (an open source binary built from the source code of VSCode). The Arduino build process will still use GCC via `arduino-cli` (see next step), `clangd` is just for language support in the editor.
  - Note: to get clangd working, you will need to symlink `build/compile_commands.json` to `compile_commands.json` (in the project root directory) after your first build. I used `ln -s $PWD/build/compile_commands.json $PWD/compile_commands.json`. An alternative would be to create a `.clangd` file.
- Install [arduino-cli](https://arduino.github.io/arduino-cli/0.19/installation/). This is necessary for the build scripts. Personally, I downloaded the the archive, extracted it to a directory, and added `arduino-cli` to my `PATH` environment variable. Alternatively, you could use the [Arduino IDE](https://www.arduino.cc/en/software), [Arduino-Makefile](https://github.com/sudar/Arduino-Makefile) (I might use this in the future), or something else.
- Install the necessary files for your board. For me and my Arduino Uno, I had to run:

   `arduino-cli config init`

   `arduino-cli core update-index`

   `arduino-cli core install arduino:avr`
- See [the usage section of this readme](#usage) for build and execution details.

## Usage

#### `make build_arduino`: 

Build the music program for an Arduino Uno. Evenutally there will be a better interface for building and including songs. **Currently, it builds an Arduino executable that loops through diagnostics (scales, Mary Had A Little Lamb, etc.).** 

The build process is a bit odd, since I need `src/ard_sound/clangd_arudino.h` to help with `clangd` language support, but I don't want it to actually be built with it. See `scripts/build_arduino.sh` for more details.

#### `make upload_arduino` 

Attempt to upload the built program to **every device in /dev/ttyACM\***. This is done because power-cycling an Arduino will often change its device file between `ttyACM0` and `ttyACM1`.  

#### `make parse`

Build the parser. This will create the executable `build/parse`.

#### `build/parse <file.abc>`

Run the parser on your computer. Eventually, this should generate C code in the form of header files. **(not yet fully implemented)**

## Architecture and Design

### Wiring Schematic
The Arduino itself should wired up similarly to the following diagram:

![Wiring Diagram](/misc/wiring/wiring_diagram.png)

(diagram made with [Fritzing](https://fritzing.org/) and [GIMP](https://www.gimp.org/))

The toggle switch is there if you want to disable the sound output without unplugging the Arduino. Very useful if you want to avoid going crazy :), but certainly not necessary. Just connect ground directly to the passive buzzer if you don't want it.

The specific digital pin that you use isn't important, but (currently) it is hardcoded to be **pin 7**.

### Software Design

C code will be generated from a program that is run beforehand on a host computer. This is done via a C program that parses an `abc` file **(this is not yet fully implemented)**. 

The main goal of this approach is to reduce executable size. Via [the Arduino website](https://www.arduino.cc/en/pmwiki.php?n=Tutorial/Memory): 

> The ATmega328 chip found on the Uno has the following amounts of memory:
> 
> `Flash  32k bytes (of which .5k is used for the bootloader)`
> 
> `SRAM   2k bytes`
> 
> `EEPROM 1k byte`

By pre-generating our "songs" via a parser, we are able to figure out beforehand exactly which notes we need, then create them in a `const` fashion by generating code. Thus, the Arduino "song" will not store each note, only a reference (pointer or index) to each one.

Whether or not this will have a significant effect on the compiled size of the program depends on the number of repeated notes of the specific song, as well as the pointer size of the platform (`16 bits` on the Uno).

This design might be re-evalutated in the future depending on performance.

## License

Unless otherwise noted, all files in this reporitory are released under the terms of the GNU General Public License version 3 or, at your choice, any later version. (GPLv3+). See [COPYING](COPYING) for more details.

## Contributors

At this time, I ([Tobias H.](https://github.com/tobias-hildebrandt)) am the sole contributor and maintainer. If you feel like contributing, send a Pull Request or shoot me a message!

## Additional Reading

- `.abc` notation: [ABC Notation Homepage](https://abcnotation.com/), [ABC examples](https://abcnotation.com/examples), [the ABC Standard](https://abcnotation.com/wiki/abc:standard:v2.1), [the ABC Plus Project](http://abcplus.sourceforge.net/)
- Formula for note frequency: [One explanation](https://pages.mtu.edu/~suits/NoteFreqCalcs.html), [another explanation](https://en.wikipedia.org/wiki/Piano_key_frequencies)
- A cool code generator: [iMatix GSL code generator](https://github.com/imatix/gsl)

