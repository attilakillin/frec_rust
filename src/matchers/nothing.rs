use regex::Regex;

use crate::{matcher::Matcher, types::Match};

use super::NothingMatcher;

impl NothingMatcher {
    pub fn new(pattern: &str) -> NothingMatcher {
        return NothingMatcher {
            original: Regex::new(pattern).unwrap()
        }
    }
}

impl Matcher for NothingMatcher {
    fn find(&self, text: &str) -> Option<Match> {
        let result = self.original.find(text);

        return match result {
            Some(content) => Some(Match::new(content.start() as isize, content.end() as isize )),
            None => None
        };
    }
}
