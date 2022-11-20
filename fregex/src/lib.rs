use matcher::Matcher;
use types::Match;

pub(crate) mod matcher;
pub(crate) mod matchers;
pub(crate) mod multimatcher;
pub(crate) mod multimatchers;
pub(crate) mod preprocessor;
pub mod types;

/// A common trait shared by the `Regex` and `MultiRegex` structs. Can be used for dynamic
/// dispatch between the two matchers.
pub trait RegexMatcher {
    /// Determines whether the given text contains any matches for the compiled pattern(s).
    fn is_match<'t>(&self, text: &'t str) -> bool;

    /// Finds the first match of the compiled pattern present
    /// in the text, or returns None if no matches are found.
    fn find<'t>(&self, text: &'t str) -> Option<Match<'t>>;
}

/// A compiled regular expression pattern matcher. Can be used to find the first
/// occurrence of an extended regular expression pattern in a given string.
/// 
/// The underlying algorithm depends on the structure of the pattern, which is
/// analyzed during the compilation phase. This implementation dynamically chooses
/// the best performing algorithm for the given pattern.
pub struct Regex<'p> {
    /// A specific matcher implementation that will be used for searching.
    matcher: Box<dyn Matcher + 'p>,
}

/// A compiled regular expression pattern matcher. Can be used to find the first
/// occurrence of any one of the supplied extended regular expression patterns
/// in a given string.
/// 
/// The underlying algorithm depends on the structures of the patterns, which are
/// analyzed during the compilation phase. This implementation dynamically chooses
/// the best performing algorithm for any given set of patterns.
pub struct MultiRegex<'p> {
    /// A specific matcher implementation that will be used for searching.
    matcher: Box<dyn Matcher + 'p>,
}
