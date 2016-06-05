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

# initial/reset state

From page 18 of gb.pdf

    0xFF05 = 0x00;
    0xFF06 = 0x00;
    0xFF07 = 0x00;
    0xFF10 = 0x80;
    0xFF11 = 0xBF;
    0xFF12 = 0xF3;
    0xFF14 = 0xBF;
    0xFF16 = 0x3F;
    0xFF17 = 0x00;
    0xFF19 = 0xBF;
    0xFF1A = 0x7F;
    0xFF1B = 0xFF;
    0xFF1C = 0x9F;
    0xFF1E = 0xBF;
    0xFF20 = 0xFF;
    0xFF21 = 0x00;
    0xFF22 = 0x00;
    0xFF23 = 0xBF;
    0xFF24 = 0x77;
    0xFF25 = 0xF3;
    0xFF26 = 0xF1;
    0xFF40 = 0x91;
    0xFF42 = 0x00;
    0xFF43 = 0x00;
    0xFF45 = 0x00;
    0xFF47 = 0xFC;
    0xFF48 = 0xFF;
    0xFF49 = 0xFF;
    0xFF4A = 0x00;
    0xFF4B = 0x00;

# only 20 commands in DMG_ROM.BIN:

Maybe there are 2-3 variants tops for each one...

    1 POP
    1 PUSH
    1 SUB
    2 CALL
    2 DAA
    2 RL
    2 RLA
    2 RRCA
    2 XOR
    3 BIT
    3 RLCA
    6 ADD
    6 CP
    7 LDH
    13 DEC
    31 RET
    51 JR
    63 INC
    239 LD
    1132 NOP
