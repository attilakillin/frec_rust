use regex::Regex;

pub mod literal;
pub mod longest;
pub mod nothing;
pub mod prefix;

pub struct LiteralMatcher<'p> {
    pattern: &'p str,
}

pub struct LongestMatcher {
    original: Regex,
    best_fragment: String,
    max_length: isize,
    length_known: bool,
}

pub struct PrefixMatcher {
    original: Regex,
    prefix: String,
}

pub struct NothingMatcher {
    original: Regex,
}
