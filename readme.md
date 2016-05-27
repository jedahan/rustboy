# rustboy

A game boy emulator, written in rust!

Inspired by [Ferris Makes Emulators](https://www.youtube.com/playlist?list=PL-sXmdrqqYYcznDg4xwAJWQgNL2gRray2)

With help from [Dissecting the Game Boy](https://www.youtube.com/watch?v=ecTQVa42sJc)

To install dependencies

    cargo install

Put some roms in `roms/`, then test their checksums with

    cargo test

It should only show 1 passing test - if you want to see that it is testing your roms, try

    cargo test -- --nocapture

To run a game

    cargo run -- roms/Super\ Mario\ Land\ \(World\).gb
