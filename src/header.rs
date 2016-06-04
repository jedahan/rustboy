use std::ops::Range;

#[derive(Debug)]
pub struct Header {
    pub name: &'static str,
    pub range: Range<usize>,
    pub format: &'static str,
}

impl Header {
    pub fn new(name: &'static str, range: Range<usize>) -> Header {
        Self::with_format(name, range, "")
    }

    pub fn with_format(name: &'static str, range: Range<usize>, format: &'static str) -> Header {
        Header {
            name: name,
            range: range,
            format: format
        }
    }
}
