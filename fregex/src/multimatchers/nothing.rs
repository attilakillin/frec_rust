use crate::{
    matcher::Matcher,
    matchers::{LiteralMatcher, LongestMatcher, PrefixMatcher, NothingMatcher},
    preprocessor::{Suggestion, Preprocessor},
    types::{Match}
};

use super::NothingMultiMatcher;


impl<'p> NothingMultiMatcher<'p, '_> {
    /// Create a new matcher with the supplied patterns.
    pub fn new(patterns: &'p [&'p str]) -> NothingMultiMatcher<'p, '_> {
        let mut matchers: Vec<Box<dyn Matcher + 'p>> = vec![];

        // For each pattern, determine type and construct matcher.
        for pattern in patterns {
            let suggestion = Preprocessor::new(pattern).determine_type().unwrap();
            let matcher: Box<dyn Matcher + 'p> = match suggestion {
                Suggestion::Literal => Box::new(LiteralMatcher::new(pattern)),
                Suggestion::Longest => Box::new(LongestMatcher::new(pattern)),
                Suggestion::Prefix => Box::new(PrefixMatcher::new(pattern)),
                Suggestion::Nothing => Box::new(NothingMatcher::new(pattern))
            };
            matchers.push(matcher);
        }
        
        return NothingMultiMatcher { matchers };
    }
}

impl<'p, 't> Matcher<'t> for NothingMultiMatcher<'p, 't> {
    /// Finds any one of the compiled patterns in the given text.
    fn find(&self, text: &'t str) -> Option<Match<'t>> {
        let mut best_match: Option<Match<'t>> = None;

        // Try matching each pattern, and store the match with the earliest start.
        for matcher in &self.matchers {
            if let Some(result) = matcher.find(text) {
                if best_match.is_none() || best_match.unwrap().start() > result.start() {
                    best_match = Some(result);
                }
            }
        }

        // Return with the best match. May be none if none of the matchers matched.
        return best_match;
    }
}
