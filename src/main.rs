mod lib;

use std::env;
use std::fs;
use std::io::Read;
#[cfg(test)]
use std::path::Path;

use lib::rom;

fn main() {
    let filename = env::args().nth(1).unwrap();
    let boot = load(String::from("DMG_ROM.bin"));
    let game: rom::Rom = load_rom(filename);

    let wram = [0; 1024];
    let vram = [0; 1024];

    println!("{}", game);
}

fn load(filepath: String) -> Vec<u8> {
    let mut file = fs::File::open(filepath).unwrap();
    let mut buffer = Vec::<u8>::new();
    file.read_to_end(&mut buffer).unwrap();
    buffer
}

fn load_rom(filepath: String) -> rom::Rom {
    rom::Rom {
        mem: load(filepath),
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
                let filepath = entry.path().to_str().unwrap().to_string();
                println!("testing {}", filepath);
                assert!(load_rom(filepath).is_valid());
            }
        }
    }
}
