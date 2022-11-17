use regex::Regex;

pub mod literal;
pub mod longest;
pub mod nothing;
pub mod prefix;

/// A literal pattern matcher implementation. Technically
/// nothing more than a wrapper for the `String::find()` method.
pub struct LiteralMatcher<'p> {
    /// The original pattern, stored as a string slice.
    pattern: &'p str,
}

/// A pattern matcher using the longest heuristic type.
pub struct LongestMatcher {
    /// A library-supplied regex compilation struct with the original pattern.
    original: Regex,
    /// The fragment used for pattern searching: the longest sequential character
    /// string without a special character in it.
    best_fragment: String,
    /// The maximum length a match can be. Not used when `length_known` is false.
    max_length: usize,
    /// Whether the maximum length of a match can be decided or not.
    length_known: bool,
}

/// A pattern matcher using the prefix heuristic type.
pub struct PrefixMatcher {
    /// A library-supplied regex compilation struct with the original pattern.
    original: Regex,
    /// The prefix fragment to use for pattern searching.
    prefix: String,
}

/// A pattern matcher where no heuristics can be used. Simply delegates the tasks
/// to the library-supplied matcher.
pub struct NothingMatcher {
    /// A library-supplied regex compilation struct with the original pattern.
    original: Regex,
}
