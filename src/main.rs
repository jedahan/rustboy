use std::env;
use std::fs;
use std::io::Read;

fn main() {
    let filename = env::args().nth(1).unwrap();
    let mut file = fs::File::open(&filename).unwrap();
    let mut buffer = Vec::new();
    let _ = file.read_to_end(&mut buffer).unwrap();

    println!("Opened {}, which is {} bytes", filename, buffer.len());

    struct Range {
        offset: usize,
        length: usize
    };

    impl Default for Range {
        fn default () -> Range {
            Range { offset: 0, length: 1 }
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
            offset: 0x134,
            length: 0x10
        }
    };

    let sgb = Header {
        name: "super game boy flag",
        range: Range {
            offset: 0x146,
            ..Default::default()
        },
        ..Default::default()
    };

    let cart_type = Header {
        name: "cart type (mappers)",
        range: Range {
            offset: 0x147,
            ..Default::default()
        },
        ..Default::default()
    };

    let rom_size = Header {
        name: "rom size",
        range: Range {
            offset: 0x148,
            ..Default::default()
        },
        ..Default::default()
    };

    let non_japanese = Header {
        name: "non_japanese",
        range: Range {
            offset: 0x14A,
            ..Default::default()
        },
        ..Default::default()
    };

    let headers = vec![name, sgb, cart_type, rom_size, non_japanese];

    for header in headers {
        let start = header.range.offset;
        let end = header.range.offset + header.range.length;
        let mut header_slice = &buffer[start..end];
        if header.format == "string" {
            println!("{}: {}", header.name, String::from_utf8_lossy(&mut header_slice));
        } else {
            println!("{}: {:?}", header.name, header_slice);
        }
    }

}
