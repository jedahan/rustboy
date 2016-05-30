use std::ops::Range;

#[derive(Debug)]
pub struct Header {
    pub name: &'static str,
    pub format: &'static str,
    pub range: Range<usize>
}

impl Default for Header {
    fn default () -> Header {
        Header {
            name: "default",
            format: "",
            range: 0..1
        }
    }
}
