# First steps

  I watched 30 minutes of [Ferris Makes Emulators](https://www.youtube.com/playlist?list=PL-sXmdrqqYYcznDg4xwAJWQgNL2gRray2). I spent 2 hours installing rust, cargo, and setting up [racer](https://github.com/phildawes/racer) and [vim-racer](https://github.com/racer-rust/vim-racer) to get rust code completion working.

# Between first steps and friday

  I got reading working, and started making structs to represent and print out headers of cartridges. Started to read the output of the compiler more carefully, and using `rustc -explain E382`. Still don't know much about the borrow checker, get things to compile.

# Friday

## 11:00

 Created a test, which just grabs all the roms and checks their checksum.
 Figured out how to make use statements compile or not with some googling, which lead to #[cfg(test)] directives.

 Had a little bit of issues with the borrow checker and lifetimes, so I think I am copying with filename: String in main::load, instead of passing around a reference. Would like to figure out how to pass around a reference.

 Would also like to figure out how to use `try!` instead of `unwrap` everywhere.

 Watched the dissected the game boy part 2 video, learned the memory mappings, which I added to notes.md

  What to do next?
    Start making [u8] slices to represent all this memory?
    Then make a map() function that maps a memory address to one of those slices?
    Then make a read() and write() that uses map?

  Bought $10 headphones, so I can watch the building an N64 emulator from scratch tutorial at southside.

## 2:00

  Started at ~30 minutes into the rust video, do I need to rebind the mut buffer in load(), or does returning the rom::Rom when binding mem do that for me?
  A quick trying of modification in main says no...

  OK, so ferris looks at the boot sequence. A quick google lead to http://gbdev.gg8.se/wiki/articles/Gameboy_Bootstrap_ROM#Contents_of_the_ROM, and I downloaded the bin, which is a dumped boot rom. I will load this file, and make the memory mapper to access it.
