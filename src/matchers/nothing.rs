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
    fn find(&self, text: &str) -> Option<Match> {
        let result = self.original.find(text);

        if let Some(content) = result {
            return Some(Match::from(content.start(), content.end()));
        } else {
            return None;
        }
    }
}
