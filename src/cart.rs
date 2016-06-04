use header;

use std::fmt;
use std::ops::Index;

use header::Header;

#[derive(Debug)]
pub struct Cart {
    pub mem: Vec<u8>,
    pub headers: Vec<header::Header>
}

impl Cart {
    pub fn new(mem: Vec<u8>) -> Cart {
        Cart {
            mem: mem,
            headers: vec![
                Header::new("entry point", 0x100..0x104),
                Header::new("logo", 0x104..0x134),
                Header::with_format("title", 0x134..0x144, "string"),
                Header::new("manufacturer", 0x13F..0x142),
                Header::new("color game boy", 0x143..0x144),
                Header::new("new licensee", 0x144..0x146),
                Header::new("super game boy", 0x146..0x147),
                Header::new("cart type", 0x147..0x148),
                Header::new("rom size", 0x148..0x149),
                Header::new("ram size", 0x149..0x14A),
                Header::new("destination", 0x14A..0x14B),
                Header::new("old licensee", 0x14B..0x14C),
                Header::new("make rom version", 0x14C..0x14D),
                Header::new("header checksum", 0x14D..0x14E),
                Header::new("global checksum", 0x14E..0x150),
                Header::new("short header", 0x134..0x14D),
                Header::new("full header", 0x100..0x14F),
            ]
        }
    }

    fn checksum(&self) -> u8 {
        self.mem[0x134..0x14D].iter().fold(0, |a: u8, &b| a.wrapping_sub(b+1))
    }

    fn global_checksum(&self) -> u16 {
        let mut sum = self.mem.iter().fold(0, |a: u16, &b| a.wrapping_add(b as u16));
        sum = sum - self.mem[0x14D] as u16 - self.mem[0x14E] as u16;
        sum
    }

    pub fn is_valid(&self) -> bool {
        self.checksum() == self.mem[0x14D]
    }
}

impl fmt::Display for Cart {
 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

     for header in &self.headers {
         if header.format == "string" {
             try!(writeln!(f, "{}: {}", header.name, String::from_utf8_lossy(&self.mem[header.range.start..header.range.end])));
         } else {
             try!(write!(f, "{}: [", header.name));
             for byte in &self.mem[header.range.start..header.range.end-1] {
                 try!(write!(f, "{:0>2X}, ", byte));
             }
             try!(writeln!(f, "{:0>2X}]", &self.mem[header.range.end]));
         }
     }

     if self.is_valid() {
         try!(writeln!(f, "checksum passed!"));
     } else {
         try!(writeln!(f, "checksum failed!"));
     }

     writeln!(f, "global checksum {:X}", self.global_checksum())

 }
}

impl Index<u16> for Cart {
    type Output = u8;

    fn index(&self, index: u16) -> &u8 {
        &self.mem[index as usize]
    }
}

impl Index<usize> for Cart {
    type Output = u8;

    fn index(&self, index: usize) -> &u8 {
        &self.mem[index]
    }
}
