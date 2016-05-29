use std::fs;
use std::io::Read;
use std::path::Path;

use cart::Cart;

pub fn rom<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut file = fs::File::open(path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    buffer
}

pub fn cart<P: AsRef<Path>>(path: P) -> Cart {
    Cart {
        mem: rom(path),
        ..Default::default()
    }
}

