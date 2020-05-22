/// examples
///
/// Span(start=2, end=5)
/// abcdefghijkl
///   ---
///
/// the end is exclusive!
/// basically, see the slice string function to see how the space relates
#[derive(Eq, PartialEq, Debug)]
pub struct Span {
    start: usize,
    end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn length(&self) -> usize {
        self.end - self.start
    }

    pub fn slice_string<'a>(&self, string: &'a str) -> Option<&'a str> {
        if self.end <= string.len() {
            Some(&string[self.start..self.end])
        } else {
            None
        }
    }
}
