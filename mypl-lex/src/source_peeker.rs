pub(crate) struct SourcePeeker<'a> {
    source: &'a str,
}

impl<'a> SourcePeeker<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source }
    }

    pub fn until(&self, offset: usize, until_char: char) -> Option<&'a str> {
        if offset >= self.source.len() {
            return None;
        }

        let offsetted = &self.source[offset..];
        offsetted
            .find(until_char)
            .map(|idx| &offsetted[0..idx + 1])
            .or(Some(offsetted))
    }
}
