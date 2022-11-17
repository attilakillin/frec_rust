use crate::matcher::Matcher;

use self::wumanber::WuManber;

pub mod literal;
pub mod nothing;
pub mod wumanber;

/// A literal pattern matcher implementation. Technically
/// nothing more than a wrapper for the Wu-Manber search algorithm.
pub struct LiteralMultiMatcher<'p> {
    /// The compiled Wu-Manber search algorithm instance.
    matcher: WuManber<'p>,
}

/*pub struct LongestMultiMatcher<'p> {
    /// The compiled Wu-Manber instance for the best pattern fragments.
    best_matcher: WuManber<'p>
}*/

pub struct NothingMultiMatcher<'p> {
    matchers: Vec<Box<dyn Matcher + 'p>>
}
