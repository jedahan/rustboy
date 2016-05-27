# hardware
  Sharp LR35902 core @ 4.19 MHz

# memory layout

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
