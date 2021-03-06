# rustboy

A game boy emulator, written in rust!

Inspired by [Ferris Makes Emulators](https://www.youtube.com/playlist?list=PL-sXmdrqqYYcznDg4xwAJWQgNL2gRray2)


# building and running

Right now, the only build requirement is [rust](rust-lang.org)

To run, the emulator expects a **dmg_rom.bin** with sha1 sum of `4ed31ec6b0b175bb109c0eb5fd3d193da823339f`, and the included **roms/test.gb**

    DEBUG=true cargo run --release

If you would like to try other roms, try

    DEBUG=true cargo run --release -- roms/my_cool_rom.gb

# thanks

- to [Dissecting the Game Boy](https://www.youtube.com/watch?v=ecTQVa42sJc) for an accessible visual overview
- [Past Raiser's opcode chart](http://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html) for a succinct chart of all opcodes
- [the gbdev wiki](http://gbdev.gg8.se/wiki/articles/Gameboy_Bootstrap_ROM) for bootstrap rom and other reference material
- [mmuszkow's game boy disassembler](https://github.com/mmuszkow/gb-disasm) for letting me understand and debug all the roms
- [ASMschool](http://gameboy.mongenel.com/asmschool.html) for dev-oriented writing
- [#rust-beginners](irc://irc.mozilla.org/rust-beginners) for being friendly
- [8080, z80, gameboy~z80, and 6502 opcode comparison](http://www.chrisantonellis.com/files/gameboy/8080_z80_6502.txt) gameboy~z80 is funky


# documentation

I am trying to verbosely write about the creation process. Please see the [development log](development_log.md) for a narrative, and [notes](notes.md) for more of a scratchpad.

Some of the more interesting commits, due to documentation and understanding of the game boy, emulators, or rust:

- [reading and printing a header from a rom](https://github.com/jedahan/rustboy/commit/46ea2281800509695aff5d40cfe4a0bb9ded53d3)
- [implementing checksum for the game boy rom](https://github.com/jedahan/rustboy/commit/950cd6832a3f301bc92c2663aee638eb866d75ee)
- [first multiline commit message](https://github.com/jedahan/rustboy/commit/3adc7060f288cbb14679d25cd4b2b0a194ee42e0)
- [what its like to stare at numbers for a long time](https://github.com/jedahan/rustboy/commit/a32784362a0e941c0b49044229d57d32f474407a)
- [how to model a virtual processor](https://github.com/jedahan/rustboy/commit/356cea58c801b7b04eab87ecbe8c26ae04c2ff16)
- [blindly copying ferris](https://github.com/jedahan/rustboy/commit/f0254ea50426258105dcf8017457687978dcefe8)
- [starting to understand differences between arrays, slices and vectors](https://github.com/jedahan/rustboy/commit/12328eb87d5eabbdccdcf297e74bdba668958873)
- *[memory mapping / difficulties implementing a trait in rust](https://github.com/jedahan/rustboy/commit/92a215f7a6746de2114332d0463e8a667c6b8689)
- *[the first version of the emulator that runs an emulated instruction!](https://github.com/jedahan/rustboy/commit/48155ecf49892a1835dd44be35a2c6a0c513b0e0)

# using

To install dependencies

    cargo install

Put some roms in `roms/`, then test their checksums with

    cargo test

It should only show 1 passing test - if you want to see that it is testing your roms, try

    cargo test -- --nocapture

To run a game

    cargo run -- DMG_ROM.bin roms/Tetris\ \(World\).gb

Note, this requires the game boy boot rom, which I cannot distribute. You can google for it though.
