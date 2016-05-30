use std::fmt;
use header;
use range;
use std::ops::Index;

#[derive(Debug)]
pub struct Cart {
    pub mem: Vec<u8>,
    pub headers: Vec<header::Header>
}

fn make_header(name: &'static str, start: usize, end: usize) -> header::Header {
    header::Header {
        name: name,
        range: start..end,
        ..Default::default()
    }
}

impl Default for Cart {
    fn default () -> Cart {
        Cart {
            mem: vec![0],
            headers: vec![
                make_header("entry point", 0x100, 0x104),
                make_header("logo", 0x104, 0x134),
                header::Header {
                    name: "title",
                    format: "string",
                    range: 0x134..0x144,
                    ..Default::default()
                },
                make_header("manufacturer", 0x13F, 0x142), // todo: string
                make_header("color game boy", 0x143, 0x144),
                make_header("new licensee", 0x144, 0x146), // todo: string
                make_header("super game boy", 0x146, 0x147),
                make_header("cart type", 0x147, 0x148),
                make_header("rom size", 0x148, 0x149),
                make_header("ram size", 0x149, 0x14A),
                make_header("destination", 0x14A, 0x14B),
                make_header("old licensee", 0x14B, 0x14C),
                make_header("make rom version", 0x14C, 0x14D),
                make_header("header checksum", 0x14D, 0x14E),
                make_header("global checksum", 0x14E, 0x150),
                make_header("short header", 0x134, 0x14D),
                make_header("full header", 0x100, 0x14F),
            ]
        }
    }
}

impl Cart {
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

    fn index<'a>(&'a self, index: u16) -> &u8 {
        &self.mem[index as usize]
    }
}
