#[derive(Debug)]
pub struct Range {
  pub start: usize,
  pub end: usize
}

impl Default for Range {
    fn default () -> Range {
        Range {
            start: 0,
            end: 1
        }
    }
}
