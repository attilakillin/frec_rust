use std::collections::HashMap;

pub mod literal;
pub mod longest;

pub struct LiteralMatcher<'p> {
    pattern: &'p str,
    //good_shifts: Vec<usize>,
    //bad_shifts: HashMap<char, usize>,
}

pub struct LongestMatcher<'p> {
    pattern: &'p str,
    best_fragment: &'p str,
}
