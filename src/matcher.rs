use crate::{
    types::{Error, Match},
    matchers::{LiteralMatcher, LongestMatcher, NothingMatcher, PrefixMatcher},
    preprocessor::{Preprocessor, Suggestion}
};

/// A trait to be implemented for each concrete matcher type.
/// Can be used to find the compiled pattern in a given text.
/// 
/// Specific matchers may be used for specific pattern structures
/// to achieve as much optimization during searching as possible.
pub trait Matcher {
    /// Find the compiled pattern in the given text.
    fn find(&self, text: &str) -> Option<Match>;
}

/// The base struct to use for regular expression matching.
pub struct Regex<'p> {
    /// A concrete matcher that implements the Matcher trait.
    matcher: Box<dyn Matcher + 'p>,
}

impl<'p> Regex<'p> {
    /// Create a new regular expression matcher from the given pattern and
    /// compilation flags.
    /// 
    /// The function determines which internal matcher works best on the
    /// given pattern and instantiates it to be used during matching.
    pub fn new(pattern: &str) -> Result<Regex, Error> {
        let parse_result = Preprocessor::new(pattern).determine_type();

        return match parse_result {
            Ok(Suggestion::Literal) => Ok(Regex {
                matcher: Box::new(LiteralMatcher::new(pattern))
            }),
            Ok(Suggestion::Longest) => Ok(Regex {
                matcher: Box::new(LongestMatcher::new(pattern))
            }),
            Ok(Suggestion::Prefix) => Ok(Regex {
                matcher: Box::new(PrefixMatcher::new(pattern))
            }),
            Ok(Suggestion::Nothing) => Ok(Regex {
                matcher: Box::new(NothingMatcher::new(pattern))
            }),
            Err(reason) => Err(reason),
        };
    }

    /// Determines whether the given text contains any matches for the compiled pattern.
    pub fn is_match(&self, text: &str) -> bool {
        return self.matcher.find(text).is_some();
    }
    
    /// Finds the first match of the compiled pattern present
    /// in the text, or returns None if no matches are found.
    pub fn find(&self, text: &str) -> Option<Match> {
        return self.matcher.find(text);
    }
}
