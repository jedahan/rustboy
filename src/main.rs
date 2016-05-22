mod lib;

use std::env;
use std::fs;
use std::io::Read;

use lib::rom;

fn main() {
    let filename = env::args().nth(1).unwrap();
    let mut file = fs::File::open(&filename).unwrap();
    let mut buffer = Vec::new();
    let _ = file.read_to_end(&mut buffer).unwrap();

    println!("Opened {}, which is {} bytes", filename, buffer.len());

    let the_rom = rom::Rom {
        mem: buffer,
        ..Default::default()
    };

    println!("{}", the_rom);
}
