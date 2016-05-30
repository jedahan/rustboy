use range;

#[derive(Debug)]
pub struct Header {
    pub name: &'static str,
    pub format: &'static str,
    pub range: range::Range
}

impl Default for Header {
    fn default () -> Header {
        Header {
            name: "default",
            format: "",
            range: range::Range { ..Default::default() }
        }
    }
}
