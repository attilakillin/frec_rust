use crate::{matchers::LongestMatcher, matcher::Matcher, types::Match};

use super::{LongestMultiMatcher, wumanber::WuManber};

impl LongestMultiMatcher {
    /// Create a new matcher with the supplied patterns.
    pub fn new(patterns: &[&str]) -> LongestMultiMatcher {
        // Build each matcher.
        let matchers: Vec<Box<LongestMatcher>> = patterns
            .iter()
            .map(|p| Box::new(LongestMatcher::new(p)))
            .collect();

        // Gather each best fragment.
        let best_fragments: Vec<&str> = matchers
            .iter()
            .map(|m| m.best_fragment())
            .collect();

        // Create Wu-Manber struct and return with a newly created matcher.
        let best_matcher = WuManber::new(&best_fragments, 2);
        return LongestMultiMatcher { matchers, best_matcher };
    }
}

impl<'t> Matcher<'t> for LongestMultiMatcher {
    /// Finds any one of the compiled patterns in the given text.
    fn find(&self, text: &'t str) -> Option<Match<'t>> {
        // The only difference in this function, and the single pattern longest matching one
        // is that this matcher finds candidates from multiple patterns, and we need to locate
        // a potential match using the correct single pattern matcher.

        // Create a mutable text slice and set global offset.
        let mut text = text;
        let mut global_offset: usize = 0;

        // This loop searches for match candidates. If a candidate is found,
        // but it is not a proper match, the start of the slice will be adjusted
        // to after the candidate's end. We iterate until there's no more text to match.
        while text.len() > 0 {
            // Search for the best fragment. If it's not present, no match can be found.
            let candidate = self.best_matcher.find(text);
            if candidate.is_none() {
                return None;
            }

            // Destructure candidate coordinates and pattern id.
            let (coords, pattern_id) = candidate.unwrap();

            // Locate the possible match.
            let (result, end) = self.matchers[pattern_id].locate_near(text, coords.start() as usize);

            // If we found something, return with the match.
            if result.is_some() {
                let content = result.unwrap();
                let match_start = content.start() + global_offset;
                let match_end = content.end() + global_offset;
                let matched_text = content.as_str();
                return Some(Match::new(match_start, match_end, matched_text));
            }

            // Else adjust search range, and continue with the next iteration.
            text = &text[end..];
            global_offset += end;
        }

        // Return none if we ran out of text to search.
        return None;
    }
}
