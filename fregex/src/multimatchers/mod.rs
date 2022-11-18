use crate::{matcher::Matcher, matchers::LongestMatcher};

use self::wumanber::WuManber;

pub mod literal;
pub mod longest;
pub mod nothing;
pub mod wumanber;

/// A literal pattern matcher implementation. Technically
/// nothing more than a wrapper for the Wu-Manber search algorithm.
pub struct LiteralMultiMatcher {
    /// The compiled Wu-Manber search algorithm instance.
    matcher: WuManber,
}

/// A multimatcher implementation that operates on patterns that can be
/// processed using the longest heuristic type.
pub struct LongestMultiMatcher {
    /// One matcher for each input pattern.
    matchers: Vec<Box<LongestMatcher>>,
    /// The compiled Wu-Manber instance for the best pattern fragments.
    best_matcher: WuManber
}

/// A multimatcher implementation that can process any and every pattern,
/// but the resulting search may not be as quick as other implementations.
pub struct NothingMultiMatcher<'p, 't> {
    /// One matcher for each input pattern.
    matchers: Vec<Box<dyn Matcher<'t> + 'p>>
}
