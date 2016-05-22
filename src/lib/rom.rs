use std::fmt;

use lib::header;
use lib::range;

pub struct Rom {
    pub mem: Vec<u8>,
    pub headers: Vec<header::Header>
}

impl Default for Rom {
    fn default () -> Rom {
        Rom {
            mem: vec![0],
            headers: vec![
                 header::Header {
                    name: "title",
                    format: "string",
                    range: range::Range {
                        start: 0x134,
                        end: 0x144
                    },
                    ..Default::default()
                },
                header::Header {
                    name: "super game boy flag",
                    range: range::Range {
                        start: 0x146,
                        end: 0x147
                    },
                    ..Default::default()
                },
                header::Header {
                    name: "cart type (mappers)",
                    range: range::Range {
                        start: 0x147,
                        end: 0x148
                    },
                    ..Default::default()
                },
                header::Header {
                    name: "rom size",
                    range: range::Range {
                        start: 0x148,
                        end: 0x149
                    },
                    ..Default::default()
                },
                header::Header {
                    name: "non_japanese",
                    range: range::Range {
                        start: 0x14A,
                        end: 0x14B
                    },
                    ..Default::default()
                },
                header::Header {
                    name: "header checksum",
                    range: range::Range {
                        start: 0x014D,
                        end: 0x014E,
                    },
                    ..Default::default()
                },
                header::Header {
                    name: "checksummed header",
                    range: range::Range {
                        start: 0x134,
                        end: 0x14D
                    },
                    ..Default::default()
                },
                header::Header {
                    name: "header",
                    range: range::Range {
                        start: 0x100,
                        end: 0x14F
                    },
                    ..Default::default()
                }
            ]
        }
    }
}

impl Rom {
    fn checksum(&self) -> u8 {
        self.mem[0x134..0x14D].iter().fold(0, |a: u8, &b| a.wrapping_sub(b+1))
    }

    fn is_valid(&self) -> bool {
        self.checksum() == self.mem[0x14D]
    }
}

impl fmt::Display for Rom {
 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

     for header in &self.headers {
         if header.format == "string" {
             try!(writeln!(f, "{}: {}", header.name, String::from_utf8_lossy(&self.mem[header.range.start..header.range.end])));
         } else {
             try!(writeln!(f, "{}: {:?}", header.name, &self.mem[header.range.start..header.range.end]));
         }
     }

     if self.is_valid() {
         writeln!(f, "checksum passed!")
     } else {
         writeln!(f, "checksum failed!")
     }

 }
}
