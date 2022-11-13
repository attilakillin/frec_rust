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
        if result.is_none() {
            return None;
        }

        let start = result.unwrap() as isize;
        return Some(Match::new(start, start + self.pattern.len() as isize));
    }
}
