
#[derive(Clone, PartialEq, Debug)]
pub struct Span {
    // Inclusive
    pub(crate) start: usize,

    // Exclusive
    pub(crate) end: usize,
}

impl From<(usize, usize)> for Span {
    fn from((start, end): (usize, usize)) -> Self {
        Self { start, end }
    }
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn get_size(&self) -> usize {
        self.end - self.start
    }
}
