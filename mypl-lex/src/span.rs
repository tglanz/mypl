#[derive(Eq, PartialEq, Debug)]
pub struct Span {
    // inclusive
    start: usize,

    // exclusive
    end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    /// total number of elements this spans over
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_new() {
        let span = Span::new(10, 12);
        assert_eq!(10, span.start);
        assert_eq!(12, span.end);
    }

    #[test]
    fn it_length() {
        let cases = [(0, 10, 10), (0, 5, 5), (2, 10, 8)];

        for (idx, &(start, end, expected)) in cases.iter().enumerate() {
            let actual = Span::new(start, end).length();
            assert_eq!(
                expected, actual,
                "case={}, span=({}, {}),  expected length={}, actual length: {}",
                idx, start, end, expected, actual
            );
        }
    }

    #[test]
    fn test_slice_string() {
        let cases = [
            (0, 11, "some string", Some("some string")),
            (0, 100, "some string", None),
            (0, 2, "some string", Some("so")),
            (0, 0, "s", Some("")),
        ];

        for (idx, &(start, end, string, expected)) in cases.iter().enumerate() {
            let actual = Span::new(start, end).slice_string(string);
            assert_eq!(
                expected, actual,
                "case={}, span=({}, {}),  expected slice={:#?}, actual slice: {:#?}",
                idx, start, end, expected, actual
            );
        }
    }
}
