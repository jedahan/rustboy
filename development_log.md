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

## 2:30

  In an ideal world, I would love to be able to just work with mem[0x100..0x7FFF] and have it transparently map to mem.game[0x000..0x7EFFF] ... or do cartridges come with the entire boot.rom?

## 3:00

  Looking at [this pdf](http://www.codeslinger.co.uk/pages/projects/gameboy/files/GB.pdf), and referencing [this awesome opcode table](http://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html), it looks like there are 8 bit opcodes, which are kinda normal to intel 8080 processors, and then 16 bit opcodes, which are some modified subset of Z80 instructions, which are 'prefixed' with 0xCB. The way you read the chart is to replace the x in one column/row with the value in the other column/row. This gives us a bit less than 2 * 255 operations we can implement.

  I have added a register array of u8, a program counter initialized to 0x0100, and a stack pointer initialized to 0xFFFE. These might have to be in one-length arrays if I want to do that magic slice/range translation in the future.

  From a cursory scan of the opcode table, it looks like instructions in the 8080 space range from 1-3 bytes, and all the Z80 instructions are 2 bytes long (+1 byte for the prefix). So maybe the first thing to do, is in fetch, we will have to look at the opcode, and figure out how many bytes to fetch, before we decide where it goes.

## 3:30

  Opened Tetris in [hecate](), and after googling, looks like 00 C3 50 01 is NOP JMP 0x0150, which makes sense (jump the header). 0x150 is JMP 028b, which then eads to an XOR A. So probably the first instruction I should implement is JMP?

  I am going to get a disassembler to read these codes a bit better (though there is probably a vim plugin?). brew search dasm and z80dasm look promising.

  OK so dasm just didnt work, and z80dasm kinda-worked, but didn't look good for the gameboy. More searching lead to [gb-disasm](https://github.com/mmuszkow/gb-disasm), compiles great on OSX, and the output is great. Starts with a NOP and jump to 0x150 as expected, and 0x150 jumps to 0x28b, which does XOR A. So we know we are reading the opcodes table correctly, hooray.

# Sunday

## 11:30-1:30

Spent two hours trying to split out tests to a subdirectory, but ended up fighting crates. Created a branch called split-out-tests that has more details of the failure. #rust-beginners on irc.mozilla.org was very helpful.
