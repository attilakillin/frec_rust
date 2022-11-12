use crate::{types::Match, matcher::Matcher};

use super::LiteralMatcher;

impl<'p> LiteralMatcher<'p> {
    pub fn new(pattern: &str) -> LiteralMatcher {
        return LiteralMatcher { pattern };
    }
}

impl<'p> Matcher for LiteralMatcher<'p> {
    fn find(&self, text: &str) -> Option<Match> {
        let result = text.find(self.pattern);

        match result {
            Some(start) => return Some(Match::new(start, start + self.pattern.len())),
            None => return None,
        }
    }
}
