use std::{collections::HashMap, cmp::{max, min}};

use crate::{matcher::{Matcher}, types::{Error, Match}};

use super::LiteralMatcher;

impl<'p> LiteralMatcher<'p> {
    pub fn new(pattern: &'p str) -> Result<LiteralMatcher, Error> {
        let result = LiteralMatcher {
            pattern: pattern,
            good_shifts: create_good_shifts(pattern),
            bad_shifts: create_bad_shifts(pattern),
        };

        return Ok(result);
    }
}

impl<'p> Matcher for LiteralMatcher<'p> {
    fn find(&self, text: &str) -> Option<Match> {
        let pattern_len = self.pattern.len();
        let mut current = 0;
        let mut shift = pattern_len;
        let mut last_suffix = 0;

        while current + shift <= text.len() {
            let mut i = pattern_len - 1;
            while i >= 0 && self.pattern.as_bytes()[i] == text.as_bytes()[current + i] {
                i -= 1;
                if last_suffix != 0 && i == pattern_len - 1 - shift {
                    i -= last_suffix;
                }
            }

            if i < 0 {
                return Some(Match {
                    start: current,
                    end: current + pattern_len,
                });
            } else {
                let v = pattern_len - 1 - i;
                let turbo_shift = last_suffix - v;

                let key = text.as_bytes()[current + i] as char;
                let value = self.bad_shifts.get(&key).unwrap_or(&pattern_len);

                let bad_shift = value - v;

                shift = max(turbo_shift, bad_shift);

                if self.good_shifts[i] >= shift {
                    last_suffix = min(pattern_len - self.good_shifts[i], v);
                } else {
                    if turbo_shift < bad_shift {
                        shift = max(shift, last_suffix + 1);
                    }
                    last_suffix = 0;
                }
            }

            current += shift;
        }

        return None;
    }
}

fn create_bad_shifts(pattern: &str) -> HashMap<char, usize> {
    let mut result = HashMap::new();

    for (i, c) in pattern.char_indices() {
        result.insert(c, i);
    }

    return result;
}

fn create_good_shifts(pattern: &str) -> Vec<usize> {
    let len = pattern.len();
    let mut suff = vec![0; len];

    suff[len - 1] = len;
    let (mut f, mut g) = (0, len - 1);
    
    // Calculate suffixes, as per the specification of the BM algorithm.
    for i in (0 ..= (len - 2)).rev() {
        if i > g && suff[len - 1 + i - f] < i - g {
            suff[i] = suff[len - 1 + i - f];
        } else {
            f = i;
            if i < g {
                g = i;
            }

            while g >= 0 && pattern.as_bytes()[g] == pattern.as_bytes()[g + len - 1 - f] {
                g -= 1;
            }

            suff[i] = f - g;
        }
    }

    // Calculate good suffix shifts.
    let mut result = vec![len; len];

    let mut j = 0;
    for i in (0 ..= (len - 1)).rev() {
        if suff[i] == i + 1 {
            while j < len - 1 - i {
                if result[j] == len {
                    result[j] = len - 1 - i;
                }
                j += 1;
            }
        }
    }

    for i in 0 ..= (len - 2) {
        result[len - 1 - suff[i]] = len - 1 - i;
    }

    return result;
}
