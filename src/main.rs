#![cfg_attr(feature="clippy", feature(plugin))]

#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate crc;

mod gameboy;
mod cpu;
mod memory;
mod cart;
mod header;
mod debug;
mod lcd;
mod window;

use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;
use crc::crc32;

fn main() {
    let boot = load_bootrom(Path::new("dmg_rom.bin"));

    let cart_path = env::args().nth(1).unwrap_or("roms/test.gb".to_string());

    let cart = load_cart(Path::new(&cart_path));
    println!("{}", cart);

    gameboy::run(boot, cart);
}

fn load_bootrom(path: &Path) -> [u8; gameboy::BOOTROM_SIZE] {
    let mut file = fs::File::open(path)
        .expect("Please download dmg_rom.bin");

    let mut buffer = [0; gameboy::BOOTROM_SIZE];
    file.read_exact(&mut buffer).unwrap();

    let dmg_rom_crc32 = 0x59c8598e;
    assert_eq!(crc32::checksum_ieee(&buffer), dmg_rom_crc32,
        "{:?} has invalid crc32, expected {:x}", path, dmg_rom_crc32);

    buffer
}

fn load_rom(path: &Path) -> Vec<u8> {
    let mut file = fs::File::open(path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    buffer
}

fn load_cart(path: &Path) -> cart::Cart {
    cart::Cart::new(load_rom(path))
}

#[test]
fn checksums() {
    println!("");
    let dir = Path::new("roms");
    if fs::metadata(dir).unwrap().is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_file() {
                // We only test against official cartridges, not homebrew
                if entry.file_name().to_string_lossy().contains("(") {
                    println!("testing {:?}", entry.file_name());
                    assert!(load_cart(entry.path()).is_valid());
                }
            }
        }
    }
}
