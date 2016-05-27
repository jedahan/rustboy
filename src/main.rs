mod lib;

use std::env;
use std::fs;
use std::io::Read;
#[cfg(test)]
use std::path::Path;

use lib::rom;

fn main() {
    let filename = env::args().nth(1).unwrap();
    let game: rom::Rom = load(filename);
    println!("{}", game);
}

fn load(filepath: String) -> rom::Rom {
    let mut file = fs::File::open(filepath).unwrap();
    let mut buffer = Vec::new();
    let _ = file.read_to_end(&mut buffer).unwrap();

    rom::Rom {
        mem: buffer,
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
                assert!(load(filepath).is_valid());
            }
        }
    }
}
