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
    fn find<'t>(&self, text: &'t str) -> Option<Match<'t>> {
        let result = text.find(self.pattern);

        if let Some(start) = result {
            let end = start + self.pattern.len();
            let matched_text = &text[start..end];
            return Some(Match::new(start, end, matched_text));
        } else {
            return None;
        }
    }
}
