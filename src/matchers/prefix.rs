use regex::Regex;

use crate::{matcher::Matcher, types::Match};

use super::PrefixMatcher;

impl PrefixMatcher {
    pub fn new(pattern: &str) -> PrefixMatcher {
        let original = Regex::new(pattern).unwrap();

        let mut prefix = String::from("");
        for c in pattern.chars() {
            if ['\\', '.', '[', '^', '$', '*', '+', '?', '(', '|', '{'].contains(&c) {
                break;
            }

            prefix.push(c);
        }

        return PrefixMatcher { original, prefix };
    }
}

impl Matcher for PrefixMatcher {
    fn find(&self, text: &str) -> Option<Match> {
        let candidate = text.find(&self.prefix);

        if candidate.is_none() {
            return None;
        }

        let start = candidate.unwrap();
        let text = &text[start..];

        let result = self.original.find(text);
        if result.is_some() {
            let content = result.unwrap();
            return Some(Match::new(
                (content.start() + start) as isize,
                (content.end() + start) as isize
            ));
        } else {
            return None;
        }
    }
}
