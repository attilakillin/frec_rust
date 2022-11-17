use crate::matcher::Matcher;
use crate::types::Match;

use super::{LiteralMultiMatcher, wumanber::WuManber};


impl LiteralMultiMatcher {
    /// Create a new matcher with the supplied patterns.
    pub fn new(patterns: &[&str]) -> LiteralMultiMatcher {
        return LiteralMultiMatcher { matcher: WuManber::new(patterns, 2) };
    }
}

impl Matcher for LiteralMultiMatcher {
    /// Finds any one of the compiled patterns in the given text.
    fn find(&self, text: &str) -> Option<Match> {
        if let Some((content, _)) = self.matcher.find(text) {
            return Some(content);
        }

        return None;
    }
}
