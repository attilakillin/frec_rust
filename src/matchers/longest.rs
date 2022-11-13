use std::cmp::{max, min};

use regex::Regex;

use crate::{types::Match, matcher::Matcher};

use super::LongestMatcher;

impl LongestMatcher {
    pub fn new(pattern: &str) -> LongestMatcher {
        let original = Regex::new(pattern).unwrap();

        let mut current = String::from("");
        let mut best_fragment = String::from("");
        let mut max_length = 0;
        let mut length_known = true;

        let mut state_escaped = false;
        let mut iter = pattern.chars();
        while let Some(c) = iter.next() {
            if state_escaped {
                current.push(c);
                max_length += 1;
                state_escaped = false;
                continue;
            }

            if c == '\\' {
                state_escaped = true;
                continue;
            }

            if !['.', '[', '+', '?', '*'].contains(&c) {
                current.push(c);
                max_length += 1;
                continue;
            }

            if c == '*' || c == '?' {
                current.pop();
            }

            if current.len() > best_fragment.len() {
                best_fragment = current;
                current = String::from("");
            }

            if c == '.' {
                max_length += 1;
                continue;
            }

            if c == '+' || c == '*' {
                length_known = false;
                continue;
            }

            if c == '?' {
                continue;
            }

            if c == '[' {
                let mut depth = 1;
                while let Some(c) = iter.next() {
                    if c == '[' {
                        depth += 1;
                    }

                    if c == ']' {
                        depth -= 1;
                    }

                    if depth == 0 {
                        break;
                    }
                }

                max_length += 1;
                continue;
            }
        }

        return LongestMatcher { original, best_fragment, max_length, length_known };
    }
}

impl Matcher for LongestMatcher {
    fn find(&self, text: &str) -> Option<Match> {
        let mut text = text;
        let mut global_offset = 0;

        while text.len() > 0 {
            let result = text.find(&self.best_fragment);

            if result.is_none() {
                return None;
            }

            let mut start = result.unwrap();
            let mut end = start + self.best_fragment.len();

            if self.length_known {
                let delta = self.max_length - (end - start) as isize;

                start = max(0 as isize, start as isize - delta) as usize;
                end = min(text.len() as isize, end as isize + delta) as usize;
            } else {
                let prev_lf = text[..start].rfind('\n');
                let next_lf = text[end..].find('\n');
                
                start = prev_lf.unwrap_or(0);
                end = next_lf.unwrap_or(text.len());
            }

            let section = &text[start..end];

            let result = self.original.find(section);

            if result.is_some() {
                global_offset += start;

                let content = result.unwrap();
                return Some(Match::new(
                    (content.start() + global_offset) as isize,
                    (content.end() + global_offset) as isize
                ));
            }

            text = &text[end..];
            global_offset += end;
        }

        return None;
    }
}
