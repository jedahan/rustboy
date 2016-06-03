use std::ops::Range;

#[derive(Debug)]
pub struct Header {
    pub name: &'static str,
    pub format: &'static str,
    pub range: Range<usize>
}

impl Header {
    pub fn new(name: &'static str, range: Range<usize>, format: &'static str) -> Header {
        Header {
            name: name,
            range: range,
            format: format
        }
    }
}
