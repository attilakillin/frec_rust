use std::collections::HashMap;

use crate::{parser::{RegexParser, ParseResult}, framework::{RegError, CompileFlags}};

use super::{BoyerMoore, Flags};

// TODO Flags are not properly handled.

pub fn literal_compile(pattern: &str, flags: CompileFlags) -> Result<BoyerMoore, RegError> {
    // Initialize result struct.
    let result = BoyerMoore {
        pattern: pattern,
        good_shifts: create_good_shifts(pattern),
        bad_shifts: create_bad_shifts(pattern),
        flags: Flags::new(flags),
    };

    // Return early if the pattern is a zero-length string.
    if pattern.is_empty() {
        result.flags.has_glob_match = true;
    }

    return Ok(result);
}

pub fn full_compile(pattern: &str, flags: CompileFlags) -> Result<BoyerMoore, RegError> {
    return match strip_specials(pattern, flags) {
        Ok(clean_pattern) => literal_compile(&clean_pattern, flags),
        Err(reason) => Err(reason),
    }
}

fn create_bad_shifts(pattern: &str) -> HashMap<char, usize> {
    let result = HashMap::new();

    for (i, c) in pattern.char_indices() {
        result[c] = i;
    }

    return result;
}

fn create_good_shifts(pattern: &str) -> Vec<usize> {
    let len = pattern.len();
    let suff = vec![0; len];

    suff[len - 1] = len;
    let (f, g) = (0, len - 1);
    
    // Calculate suffixes, as per the specification of the BM algorithm.
    for i in (0 ..= (len - 2)).rev() {
        if i > g && suff[len - 1 + i - f] < i - g {
            suff[i] = suff[len - 1 + i - f];
        } else {
            f = i;
            if i < g {
                g = i;
            }

            while g >= 0 && pattern[g] == pattern[g + len - 1 - f] {
                g -= 1;
            }

            suff[i] = f - g;
        }
    }

    // Calculate good suffix shifts.
    let result = vec![len; len];

    let j = 0;
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

fn strip_specials(pattern: &str, flags: CompileFlags) -> Result<String, RegError> {
    // TODO Need to set specific flags
    // Handle ^ and $
    let clean_pattern = String::new();

    let parser = RegexParser::new(flags.extended);

    for c in pattern.chars() {
        match parser.parse(c) {
            ParseResult::Character(_) => clean_pattern.push(c),
            ParseResult::Nothing => (),
            _ => return RegError::BadPattern,
        }
    }

    return Ok(clean_pattern);
}
