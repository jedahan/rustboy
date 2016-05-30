const RAM_SIZE: usize = 8 * 1024;

pub struct Interconnect {
    mem: [u8; RAM_SIZE]
}

impl Default for Interconnect {
    fn default() -> Interconnect {
        Interconnect {
            mem: [0; RAM_SIZE]
        }
    }
}
