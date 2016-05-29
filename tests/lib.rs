#[cfg(test)]
use std::path::Path;
use std::fs;

extern crate rustboylib;
use rustboylib::load::cart;

#[test]
fn checksums() {
    let dir = Path::new("roms");
    if fs::metadata(dir).unwrap().is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_file() {
                let filepath = entry.path();
                println!("testing {:?}", filepath);
                assert!(rustboylib::load::cart(filepath).is_valid());
            }
        }
    }
}
