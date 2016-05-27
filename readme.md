# rustboy

A game boy emulator, written in rust!

Inspired by [Ferris Makes Emulators](https://www.youtube.com/playlist?list=PL-sXmdrqqYYcznDg4xwAJWQgNL2gRray2)

To install dependencies

    cargo install

Put some roms in `roms/`, then test their checksums with

    cargo test

To run a game

    cargo run -- roms/Super\ Mario\ Land\ \(World\).gb
