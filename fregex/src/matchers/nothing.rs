use regex::Regex;

use crate::{matcher::Matcher, types::Match};

use super::NothingMatcher;

impl NothingMatcher {
    /// Create a new matcher with the supplied pattern.
    pub fn new(pattern: &str) -> NothingMatcher {
        return NothingMatcher {
            original: Regex::new(pattern).unwrap()
        }
    }
}

impl Matcher for NothingMatcher {
    /// Find the compiled pattern in the given text.
    fn find<'t>(&self, text: &'t str) -> Option<Match<'t>> {
        let result = self.original.find(text);

        if let Some(content) = result {
            let start = content.start();
            let end = content.end();
            let matched_text = &text[start..end];
            return Some(Match::new(start, end, matched_text));
        } else {
            return None;
        }
    }
}
