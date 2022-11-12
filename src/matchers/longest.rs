use crate::{types::Match, matcher::Matcher};

use super::LongestMatcher;

impl<'p> LongestMatcher<'p> {
    pub fn new(pattern: &'p str) -> LongestMatcher {
        


        return LongestMatcher {
            pattern: pattern,
            best_fragment: pattern,
        };
    }
}

impl<'p> Matcher for LongestMatcher<'p> {
    fn find(&self, text: &str) -> Option<Match> {
        let result = text.find(self.pattern);

        match result {
            Some(start) => return Some(Match::new(start, start + self.pattern.len())),
            None => return None,
        }
    }
}
