use std::collections::HashMap;

use regex::Match;

use crate::framework::{CompileFlags, RegError};

pub mod compile;
pub mod search;

pub struct BoyerMoore <'a> {
    pattern: &'a str,
    good_shifts: Vec<usize>,
    bad_shifts: HashMap<char, usize>,
    flags: Flags,
}

struct Flags {
    has_glob_match: bool,
    has_bol_anchor: bool,
    has_eol_anchor: bool,

    is_icase_set: bool,
    is_nosub_set: bool,
    is_nline_set: bool,
}

impl <'a> BoyerMoore <'a> {
    
    pub fn compile(pattern: &str, flags: CompileFlags, literal: bool) -> Result<Self, RegError> {
        if literal {
            return compile::literal_compile(pattern, flags);
        } else {
            return compile::full_compile(pattern, flags);
        }
    }

    pub fn search(self, text: &str, flags: u32) -> Option<Match> {
        return None;
    }
}

impl Flags {
    fn new(flags: CompileFlags) -> Self {
        return Self {
            has_glob_match: false,
            has_bol_anchor: false,
            has_eol_anchor: false,

            is_icase_set: false,
            is_nosub_set: false,
            is_nline_set: false,
        }
    }
}
