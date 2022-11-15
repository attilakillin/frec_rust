use regex::Regex;

use crate::{matcher::Matcher, types::Match};

use super::PrefixMatcher;

impl PrefixMatcher {
    /// Create a new matcher with the supplied pattern.
    pub fn new(pattern: &str) -> PrefixMatcher {
        let original = Regex::new(pattern).unwrap();

        // Char by char, construct the longest literal prefix fragment possible.
        let mut prefix = String::from("");
        for c in pattern.chars() {
            if ['\\', '.', '[', '^', '$', '*', '+', '?', '(', '|', '{'].contains(&c) {
                break;
            }

            prefix.push(c);
        }

        // Return with the result.
        return PrefixMatcher { original, prefix };
    }
}

impl Matcher for PrefixMatcher {
    /// Find the compiled pattern in the given text.
    fn find(&self, text: &str) -> Option<Match> {
        // Find a candidate based on our prefix string.
        let candidate = text.find(&self.prefix);
        if candidate.is_none() {
            return None;
        }

        // Run the original matcher on the remainder of the text.
        let start = candidate.unwrap();
        let result = self.original.find(&text[start..]);

        // Return the (correctly offset) result.
        if let Some(content) = result {
            return Some(Match::from(content.start() + start, content.end() + start));
        } else {
            return None;
        }
    }
}
