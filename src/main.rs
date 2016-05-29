mod lib;

use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;

use lib::cart;
use lib::cpu;

fn main() {
    let boot_rom_file_name = env::args().nth(1).unwrap();
    let boot = load_rom(Path::new(&boot_rom_file_name));
    println!("boot: {:?}", boot);

    let cart_rom_file_name = env::args().nth(2).unwrap();
    let cart = load_cart(Path::new(&cart_rom_file_name));
    println!("{}", cart);

    let mut cpu = cpu::Cpu::default();
    cpu.reset();
    println!("{}", cpu);
}

fn load_rom<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut file = fs::File::open(path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    buffer
}

fn load_cart<P: AsRef<Path>>(path: P) -> cart::Cart {
    cart::Cart {
        mem: load_rom(path),
        ..Default::default()
    }
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
