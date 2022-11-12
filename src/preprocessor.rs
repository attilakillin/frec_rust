use regex::Regex;

use crate::{types::Error};

/// An object that can be used to preprocess a pattern string and decide
/// the best matching method to use during the pattern matching.
pub struct Preprocessor<'p> {
    /// The pattern string itself.
    pattern: &'p str
}

/// Contains the various heuristic implementations that can be used for pattern matching.
pub enum Suggestion {
    Literal,
    Longest,
    Prefix,
    Nothing,
}

impl<'p> Preprocessor<'p> {
    /// Creates a new instance from the given pattern and flags.
    pub fn new(pattern: &str) -> Preprocessor {
        return Preprocessor { pattern };
    }

    /// Preprocesses the stored pattern, and determines which heuristic should be used
    /// during text searching. May return an error, if the pattern is not a valid pattern.
    pub fn determine_type(&self) -> Result<Suggestion, Error> {
        if Regex::new(self.pattern).is_err() {
            return Err(Error::Syntax("Preliminary compile check failed."));
        }

        let mut is_literal = true;
        let mut is_longest = true;

        let mut state_escaped = false;

        for c in self.pattern.chars() {
            if c == '\\' {
                state_escaped = !state_escaped;
            }

            match c {
                '.' | '[' | '^' | '$' | '*' => is_literal &= state_escaped,
                '+' | '?' | '(' | '|' | '{' => is_literal &= !(state_escaped),
                _ => (),
            }

            match c {
                '*' => is_longest &= state_escaped,
                '{' | '+' | '?' | '|' => is_longest &= !(state_escaped),
                _ => (),
            }
        }

        if is_literal {
            return Ok(Suggestion::Literal);
        } else if is_longest {
            return Ok(Suggestion::Longest);
        } else {
            return Ok(Suggestion::Nothing);
        }
    }
}
