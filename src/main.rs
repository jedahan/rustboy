use std::env;
use std::fs;
use std::io::Read;

fn checksum(data: &[u8], check: u8) -> bool {
      check == data.iter().fold(0, |acc: u8, &x| acc.wrapping_sub(x+1))
}

fn main() {
    let filename = env::args().nth(1).unwrap();
    let mut file = fs::File::open(&filename).unwrap();
    let mut buffer = Vec::new();
    let _ = file.read_to_end(&mut buffer).unwrap();

    println!("Opened {}, which is {} bytes", filename, buffer.len());

    struct Range {
      start: usize,
      end: usize
    }

    impl Default for Range {
        fn default () -> Range {
            Range {
                start: 0,
                end: 1
            }
        }
    };

    struct Header {
        name: &'static str,
        format: &'static str,
        range: Range
    };

    impl Default for Header {
        fn default () -> Header {
            Header {
                name: "default",
                format: "",
                range: Range { ..Default::default() }
            }
        }
    };

    let name = Header {
        name: "title",
        format: "string",
        range: Range {
            start: 0x134,
            end: 0x144
        },
        ..Default::default()
    };

    let sgb = Header {
        name: "super game boy flag",
        range: Range {
            start: 0x146,
            end: 0x147
        },
        ..Default::default()
    };

    let cart_type = Header {
        name: "cart type (mappers)",
        range: Range {
            start: 0x147,
            end: 0x148
        },
        ..Default::default()
    };

    let rom_size = Header {
        name: "rom size",
        range: Range {
            start: 0x148,
            end: 0x149
        },
        ..Default::default()
    };

    let non_japanese = Header {
        name: "non_japanese",
        range: Range {
            start: 0x14A,
            end: 0x14B
        },
        ..Default::default()
    };

    let header_checksum = Header {
        name: "header checksum",
        range: Range {
            start: 0x014D,
            end: 0x014E,
        },
        ..Default::default()
    };

    let header_checksum_payload = Header {
        name: "checksummed header",
        range: Range {
            start: 0x134,
            end: 0x14D
        },
        ..Default::default()
    };

    let header = Header {
        name: "header",
        range: Range {
            start: 0x100,
            end: 0x14F
        },
        ..Default::default()
    };

    let header_slice = &buffer[header_checksum_payload.range.start..header_checksum_payload.range.end];
    let header_checksum_value = &buffer[header_checksum.range.start..header_checksum.range.end];

    if checksum(header_slice, header_checksum_value[0]) {
        println!("checksum success!");
    } else {
        println!("checksum FAIL TT");
    }

    let headers = vec![header_checksum_payload, header, name, sgb, cart_type, rom_size, non_japanese, header_checksum];

    for header in headers {
        let mut header_slice = &buffer[header.range.start..header.range.end];
        if header.format == "string" {
            println!("{}: {}", header.name, String::from_utf8_lossy(&mut header_slice));
        } else {
            println!("{}: {:?}", header.name, header_slice);
        }
    }

}
