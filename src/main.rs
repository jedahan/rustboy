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

fn main() {
    let boot_path = match env::args().nth(1) {
        Some(path) => &path,
        None => "dmg_rom.bin"
    };

    let boot = load_bootrom(Path::new(&boot_path));

    let cart_path = match env::args().nth(2) {
        Some(path) => &path,
        None => "roms/test.gb"
    };

    let cart = load_cart(Path::new(&cart_path));
    println!("{}", cart);

    gameboy::run(boot, cart);
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
    println!("");
    let dir = Path::new("roms");
    if fs::metadata(dir).unwrap().is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_file() {
                let filepath = entry.path();
                // We only test against official cartridges, not homebrew
                if filepath.file_name().unwrap().to_string_lossy().contains("(") {
                    println!("testing {:?}", filepath);
                    assert!(load_cart(filepath).is_valid());
                }
            }
        }
    }
}
