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
        // We try to properly compile the pattern first, and see if it succeeds.
        if Regex::new(self.pattern).is_err() {
            return Err(Error::Syntax("Preliminary compile check failed."));
        }

        // Inner state variables.
        let mut is_literal = true;
        let mut is_longest = true;
        let mut is_prefix = true;
        let mut state_escaped = false;

        // We go through the pattern char by char.
        for (i, c) in self.pattern.char_indices() {
            // A '\' changes the escaped state.
            if c == '\\' {
                state_escaped = !state_escaped;
            }

            // We can only use the prefix heuristic if the first two chars aren't special chars.
            if (i == 0 || i == 1) && ['\\', '.', '[', '^', '$', '*', '+', '?', '(', '|', '{'].contains(&c) {
                is_prefix = false;
            }

            // The presence of the following special characters might prevent us
            // from using the literal and longest heuristics.
            if !state_escaped {
                if is_literal && ['.', '[', '^', '$', '*', '+', '?', '(', '|', '{'].contains(&c) {
                    is_literal = false;
                }

                if is_longest && ['*', '+', '?', '(', '|', '{'].contains(&c) {
                    is_longest = false;
                }
            }
        }

        // Return the correct suggestion.
        return match () {
            () if is_literal => Ok(Suggestion::Literal),
            () if is_longest => Ok(Suggestion::Longest),
            () if is_prefix => Ok(Suggestion::Prefix),
            () => Ok(Suggestion::Nothing)
        };
    }
}
