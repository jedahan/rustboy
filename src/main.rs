#![feature(alloc_system)]
extern crate alloc_system;

mod gameboy;
mod cpu;
mod memory;
mod cart;
mod header;
mod screen;

use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;

fn main() {
    let boot_rom_file_name = env::args().nth(1).unwrap();
    let boot = load_bootrom(Path::new(&boot_rom_file_name));

    let cart_rom_file_name = env::args().nth(2).unwrap();
    let cart = load_cart(Path::new(&cart_rom_file_name));
    println!("{}", cart);

    let mut gameboy: gameboy::GameBoy = gameboy::GameBoy::new(boot, cart);
    gameboy.reset();
    println!("{}", gameboy);
    gameboy.run();
}

fn load_bootrom<P: AsRef<Path>>(path: P) -> [u8; gameboy::BOOTROM_SIZE] {
    let mut file = fs::File::open(path).unwrap();
    let mut buffer = [0; gameboy::BOOTROM_SIZE];
    file.read_exact(&mut buffer).unwrap();
    buffer
}

fn load_rom<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut file = fs::File::open(path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    buffer
}

fn load_cart<P: AsRef<Path>>(path: P) -> cart::Cart {
    cart::Cart::new(load_rom(path))
}

#[test]
fn checksums() {
    let dir = Path::new("roms");
    if fs::metadata(dir).unwrap().is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_file() {
                let filepath = entry.path();
                println!("testing {:?}", filepath);
                assert!(load_cart(filepath).is_valid());
            }
        }
    }
}
