# rustboy

A game boy emulator, written in rust!

Inspired by [Ferris Makes Emulators](https://www.youtube.com/playlist?list=PL-sXmdrqqYYcznDg4xwAJWQgNL2gRray2)

# thanks

  to [Dissecting the Game Boy](https://www.youtube.com/watch?v=ecTQVa42sJc) for an accessible visual overview,
  [Past Raiser's opcode chart](http://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html) for a succinct chart of all opcodes,
  [the gbdev wiki](http://gbdev.gg8.se/wiki/articles/Gameboy_Bootstrap_ROM) for bootstrap rom and other reference material,
  [mmuszkow's game boy disassembler](https://github.com/mmuszkow/gb-disasm) for letting me understand and debug all the roms,
  [ASMschool](http://gameboy.mongenel.com/asmschool.html) for dev-oriented writing


# documentation

I am trying to verbosely write about the creation process. Please see the [development log](development_log.md) for a narrative, and [notes](notes.md) for more of a scratchpad.

# using

To install dependencies

    cargo install

Put some roms in `roms/`, then test their checksums with

    cargo test

It should only show 1 passing test - if you want to see that it is testing your roms, try

    cargo test -- --nocapture

To run a game

    cargo run -- DMG_ROM.bin roms/Super\ Mario\ Land\ \(World\).gb

Note, this requires the game boy boot rom, which I cannot distribute. You can google for it though.
