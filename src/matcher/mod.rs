use regex::{Regex, Match};
use crate::{boyermoore::BoyerMoore, heuristic::Heuristic};

pub struct Matcher {
    default: Regex,
    boyer_moore: Option<BoyerMoore>,
    heuristic: Option<Heuristic>,
}

impl Matcher {
    pub fn compile(pattern: &str, flags: u32) -> Result<Self, String> {
        // Create default matcher, if this fails, we have to abort.
        let default = match Regex::new(pattern) {
            Ok(value) => value,
            Err(error) => return Err(error.to_string()),
        };

        // Create Boyer-Moore matcher. Use None if compilation failed.
        let boyer_moore = match BoyerMoore::compile(pattern, flags) {
            Ok(value) => Some(value),
            Err(_) => None,
        };

        // If we can't use the Boyer-Moore matcher, try using heuristics.
        let heuristic = None;
        if boyer_moore.is_none() /* TODO And not REG_LITERAL */ {
            heuristic = match Heuristic::compile(pattern, flags) {
                Ok(value) => Some(value),
                Err(_) => None,
            }
        }

        return Ok(Matcher { default, boyer_moore, heuristic });
    }

    pub fn search(self, text: &str, flags: u32) -> Option<Match> {
        // If the Boyer-Moore struct can be used, use that.
        if let Some(matcher) = self.boyer_moore {
            return matcher.search(text, flags);
        }

        // Else if the heuristic struct exists, use that.
        if let Some(matcher) = self.heuristic {
            return matcher.search(text, flags);
        }

        // Else use the built-in matcher.
        return self.default.find(text);
    }
}

pub struct MultiMatcher {

}

impl MultiMatcher {
    pub fn compile(self, patterns: &[&str], flags: u32) {

    }

    pub fn search(self, text: &str, flags: u32) {

    }
}
