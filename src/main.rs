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

    struct Header {
        name: &'static str,
        range: Range
    };

    let name_header = Header {
        name: "title",
        range: Range {
            offset: 0x134,
            length: 0x10
        }
    };

    let headers = vec![name_header];

    for header in headers {
        let start = header.range.offset;
        let end = header.range.offset + header.range.length;
        let mut header_slice = &buffer[start..end];
        println!("{}: {}", header.name, String::from_utf8_lossy(&mut header_slice));
    }

}
