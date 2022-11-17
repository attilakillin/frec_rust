use crate::matcher::Matcher;
use crate::types::Match;

use super::{LiteralMultiMatcher, wumanber::WuManber};


impl<'p> LiteralMultiMatcher<'p> {
    /// Create a new matcher with the supplied patterns.
    pub fn new(patterns: &'p [&'p str]) -> LiteralMultiMatcher<'p> {
        return LiteralMultiMatcher { matcher: WuManber::new(patterns, 2) };
    }
}

impl<'p> Matcher for LiteralMultiMatcher<'p> {
    /// Finds any one of the compiled patterns in the given text.
    fn find(&self, text: &str) -> Option<Match> {
        return self.matcher.find(text);
    }
}
