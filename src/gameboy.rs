use std::env;
use std::thread;
use std::sync::{Arc, RwLock};

use cpu;
use cart;
use memory;
use debug;
use lcd;
use window;

pub const BOOTROM_SIZE: usize = 256;

pub fn run(boot: [u8; BOOTROM_SIZE], cart: cart::Cart) {
    let memory = Arc::new(RwLock::new(memory::Memory::new(boot, cart)));

    let memory_ref = memory.clone();
    let mut screen: Box<window::Drawable> = match env::var("DEBUG") {
        Ok(_) => Box::new(debug::DebugScreen::new(160, 288, memory_ref)),
        _ => Box::new(lcd::LcdScreen::new(160, 144, memory_ref)),
    };
    let mut cpu = cpu::Cpu::new(memory);

    let _ = thread::spawn(move || {
        cpu.run();
    });

    screen.run();
}
