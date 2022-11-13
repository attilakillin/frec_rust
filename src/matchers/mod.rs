use regex::Regex;

pub mod literal;
pub mod longest;

pub struct LiteralMatcher<'p> {
    pattern: &'p str,
}

pub struct LongestMatcher {
    original: Regex,
    best_fragment: String,
    max_length: isize,
}
