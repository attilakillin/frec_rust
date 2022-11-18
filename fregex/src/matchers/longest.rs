use std::cmp::min;

use regex::Regex;

use crate::{types::Match, matcher::Matcher};

use super::LongestMatcher;

impl<'t> LongestMatcher {
    /// Create a new matcher with the supplied pattern.
    pub fn new(pattern: &str) -> LongestMatcher {
        // Create library-supplied matcher.
        let original = Regex::new(pattern).unwrap();

        // Create properties of the final matcher.
        let mut best_fragment = String::from("");
        let mut max_length = 0;
        let mut length_known = true;

        // Create state variables.
        let mut current = String::from("");
        let mut state_escaped = false;
        let mut iter = pattern.chars();

        // While there's a character to parse from the pattern.
        while let Some(c) = iter.next() {
            // If something was escaped, push to the current fragment and continue.
            if state_escaped {
                current.push(c);
                max_length += 1;
                state_escaped = false;
                continue;
            }

            // On a '\', set the escaped flag and continue.
            if c == '\\' {
                state_escaped = true;
                continue;
            }

            // Unless a char is a recognized special char, push to the current fragment
            // and continue.
            if !['.', '[', '+', '?', '*'].contains(&c) {
                current.push(c);
                max_length += 1;
                continue;
            }

            // The remainder of this loop handles the special characters mentioned above.

            // If the modifier makes the previous character optional, pop it from the fragment.
            if c == '*' || c == '?' {
                current.pop();
            }

            // With every special character, the current fragment ends. Save it if it is
            // longer than our current best fragment.
            if current.len() > best_fragment.len() {
                best_fragment = current;
                current = String::from("");
            }

            // If the char was '.', we also increase the max length.
            if c == '.' {
                max_length += 1;
                continue;
            }

            // If it was a '+' or a '*', we now can't know the length of a match.
            if c == '+' || c == '*' {
                length_known = false;
                continue;
            }

            // If it was a '?', there's nothing to do.
            if c == '?' {
                continue;
            }

            // If it was a '[', we parse the text up until the closing bracket.
            // The characters inside are all different options, and as such, we
            // cannot create a fragment from them.
            if c == '[' {
                let mut inside_escaped = false;
                while let Some(c) = iter.next() {
                    // An escaped character can't be the closing bracket.
                    if inside_escaped {
                        inside_escaped = false;
                        continue;
                    }

                    // Manage escaped state.
                    if c == '\\' {
                        inside_escaped = true;
                    }

                    // Break on a closing bracket.
                    if c == ']' {
                        break;
                    }
                }

                max_length += 1;
                continue;
            }
        }

        // If we processed the whole string and there's still an unfinished fragment
        // stored in the current string, check that too.
        if current.len() > best_fragment.len() {
            best_fragment = current;
        }

        // Return a matcher with the properties created above.
        return LongestMatcher { original, best_fragment, max_length, length_known };
    }

    /// Returns the best fragment stored in the matcher.
    pub fn best_fragment(&self) -> &str {
        return &self.best_fragment;
    }

    /// Given a text and a potential match candidate, try to locate the full match.
    /// The second return value signifies the position before which no more matches can occur.
    pub fn locate_near(&self, text: &'t str, pos: usize) -> (Option<Match<'t>>, usize) {
        // Set the start and end coordinates.
        let mut start = pos;
        let mut end = start + self.best_fragment.len();

        if self.length_known {
            // If we know the length of the pattern, broaden the matching range
            // with the following delta.
            let delta = self.max_length - (end - start);

            start = if start < delta { 0 } else { start - delta };
            end = min(text.len(), end + delta);
        } else {
            // If we don't know the length, broaden the matching range to the
            // entire line the candidate was found in.
            start = text[..start].rfind('\n').unwrap_or(0);
            end = text[end..].find('\n').unwrap_or(text.len());
        }

        // Now we try using the original matcher on this excerpt of the text.
        let result = self.original.find(&text[start..end]);

        // If we found something, return with the match, else return with none.
        if result.is_some() {
            let content = result.unwrap();
            let match_start = content.start() + start;
            let match_end = content.end() + start;
            let matched_text = &text[match_start..match_end];
            return (Some(Match::new(match_start, match_end, matched_text)), end);
        } else {
            return (None, end);
        }
    }
}

impl<'t> Matcher<'t> for LongestMatcher {
    /// Find the compiled pattern in the given text.
    fn find(&self, text: &'t str) -> Option<Match<'t>> {
        // Create a mutable text slice and set global offset.
        let mut text = text;
        let mut global_offset: usize = 0;

        // This loop searches for match candidates. If a candidate is found,
        // but it is not a proper match, the start of the slice will be adjusted
        // to after the candidate's end. We iterate until there's no more text to match.
        while text.len() > 0 {
            // Search for the best fragment. If it's not present, no match can be found.
            let result = text.find(&self.best_fragment);
            if result.is_none() {
                return None;
            }

            // Locate the possible match.
            let (result, end) = self.locate_near(text, result.unwrap());

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
