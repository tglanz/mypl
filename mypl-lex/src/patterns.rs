use regex::Regex;

pub(crate) struct Patterns {
    pub whitespace: Regex,
    pub comment: Regex,
}

impl Default for Patterns {
    fn default() -> Patterns {
        Patterns {
            comment: Regex::new("^//(.*)\n*").unwrap(),
            whitespace: Regex::new(r"^[\t\n\r]+").unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_whitespace_no_match() {
        for &curr in &["abc", "a\t\r\nb"] {
            let actual = Patterns::default()
                .whitespace
                .captures(curr)
                .and_then(|c| c.get(0))
                .map(|m| m.as_str());
            assert_eq!(None, actual, "shouldn't have matched {:?}", curr);
        }
    }

    #[test]
    fn it_whitespace_match_whole() {
        for &curr in &[
            "\t", "\r", "\n", "\t\t\t", "\r\r\r", "\n\n\n", "\t\r\n", "\r\n", "\t\r",
        ] {
            let actual = Patterns::default()
                .whitespace
                .captures(curr)
                .and_then(|c| c.get(0))
                .map(|c| c.as_str());
            assert_eq!(Some(curr), actual);
        }
    }

    #[test]
    fn it_whitespace_match_prefix_1() {
        let actual = Patterns::default()
            .whitespace
            .captures("\t\r\nabcabcabc")
            .and_then(|c| c.get(0))
            .map(|m| m.as_str());
        assert_eq!(Some("\t\r\n"), actual);
    }

    #[test]
    fn it_whitespace_match_prefix_2() {
        let actual = Patterns::default()
            .whitespace
            .captures("\t\r\nabc\n\r\tabc\t\n\r")
            .and_then(|c| c.get(0))
            .map(|m| m.as_str());
        assert_eq!(Some("\t\r\n"), actual);
    }

    #[test]
    fn it_comment_match_1() {
        let captures = Patterns::default()
            .comment
            .captures("// some comment\r\n")
            .unwrap();
        assert_eq!("// some comment\r\n", captures.get(0).unwrap().as_str());
        assert_eq!(" some comment\r", captures.get(1).unwrap().as_str());
    }

    #[test]
    fn it_comment_match_2() {
        let captures = Patterns::default()
            .comment
            .captures("// this is some comment")
            .unwrap();
        assert_eq!("// this is some comment", captures.get(0).unwrap().as_str());
        assert_eq!(" this is some comment", captures.get(1).unwrap().as_str());
    }

    #[test]
    fn it_comment_match_3() {
        let captures = Patterns::default()
            .comment
            .captures("// this\r\n// is \r\n// mutliline")
            .unwrap();
        assert_eq!("// this\r\n", captures.get(0).unwrap().as_str());
        assert_eq!(" this\r", captures.get(1).unwrap().as_str());
    }

    #[test]
    fn it_comment_no_match() {
        for &curr in &[
            "asd",
            "something",
            "some\r\nmulti\r\nline",
            "some\r\n// other\r\n// multiline",
        ] {
            let captures = Patterns::default().comment.captures(curr);
            let message = format!("captured non comment: {:?}. captured: {:?}", curr, captures);
            assert!(captures.is_none(), message);
        }
    }
}
