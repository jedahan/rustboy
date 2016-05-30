use gameboy;
const RAM_SIZE: usize = 8 * 1024;

pub struct Interconnect {
    boot: [u8; gameboy::BOOTROM_SIZE],
    wram: [u8; RAM_SIZE],
    vram: [u8; RAM_SIZE]
}

impl Interconnect {
    pub fn new(boot: [u8; gameboy::BOOTROM_SIZE]) -> Interconnect {
        Interconnect {
            boot: boot,
            vram: [0; RAM_SIZE],
            wram: [0; RAM_SIZE]
        }
    }
}
