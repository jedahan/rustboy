# First steps

  I watched 30 minutes of [Ferris Makes Emulators](https://www.youtube.com/playlist?list=PL-sXmdrqqYYcznDg4xwAJWQgNL2gRray2). I spent 2 hours installing rust, cargo, and setting up [racer](https://github.com/phildawes/racer) and [vim-racer](https://github.com/racer-rust/vim-racer) to get rust code completion working.

# Between first steps and friday

  I got reading working, and started making structs to represent and print out headers of cartridges. Started to read the output of the compiler more carefully, and using `rustc -explain E382`. Still don't know much about the borrow checker, get things to compile.

# Friday

 Created a test, which just grabs all the roms and checks their checksum.
 Figured out how to make use statements compile or not with some googling, which lead to #[cfg(test)] directives.

 Had a little bit of issues with the borrow checker and lifetimes, so I think I am copying with filename: String in main::load, instead of passing around a reference. Would like to figure out how to pass around a reference.

 Would also like to figure out how to use `try!` instead of `unwrap` everywhere.



 Watched the dissected the game boy part 2 video, learned:

 0x0000..0x7FFF => cart
 0x8000..0xFFFF => other stuff

   0x8000..0x9FFF => video ram
   0xA000..0xBFFF => extra ram
   0xC000..0xDFFF => working ram (stack/heap?)
   0xE000..0xFDFF => shadow ram (mostly a copy of the working ram)
     0xFE00..0xFE9F => OAM/sprite attr table
     0xFEAF..0xFEFF => ???
     0xFF00..0xFF7F => hardware (screen, sound, buttons, timer)
     0xFF00 => input byte ( _ _ 0 1 D U L R ) // on falling edge of bit 5
     0xFF00 => input byte ( _ _ 1 0 S s B A ) // on falling edge of bit 4
     0xFF80..0xFFFE => highram (in cpu, super fast)
     0xFFFF => interrupt enable/disable

  What to do next?
    Start making [u8] slices to represent all this memory?
    Then make a map() function that maps a memory address to one of those slices?
    Then make a read() and write() that uses map?

  Bought $10 headphones, so I can watch the building an N64 emulator from scratch tutorial at southside.


  Started at ~30 minutes into the rust video, do I need to rebind the mut buffer in load(), or does returning the rom::Rom when binding mem do that for me?
  A quick trying of modification in main says no...


  OK, so ferris looks at the boot sequence. A quick google lead to http://gbdev.gg8.se/wiki/articles/Gameboy_Bootstrap_ROM#Contents_of_the_ROM, and I downloaded the bin, which is a dumped boot rom. I will load this file, and make the memory mapper to access it.
