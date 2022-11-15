use crate::{types::Match, matcher::Matcher};

use super::LiteralMatcher;

impl<'p> LiteralMatcher<'p> {
    /// Create a new matcher with the supplied pattern.
    pub fn new(pattern: &str) -> LiteralMatcher {
        return LiteralMatcher { pattern };
    }
}

impl<'p> Matcher for LiteralMatcher<'p> {
    /// Find the compiled pattern in the given text.
    fn find(&self, text: &str) -> Option<Match> {
        let result = text.find(self.pattern);

        if let Some(start) = result {
            return Some(Match::from(start, start + self.pattern.len()));
        } else {
            return None;
        }
    }
}
