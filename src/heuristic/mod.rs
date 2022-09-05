use regex::Match;

use crate::boyermoore::BoyerMoore;

pub struct Heuristic {
    compiled_literal: BoyerMoore,
    max_length: Option<usize>,
    mode: HeuristicMode,
}

pub enum HeuristicMode {

}

impl Heuristic {
    pub fn compile(pattern: &str, flags: u32) -> Result<Self, String> {
        return Ok(Heuristic {});
    }

    pub fn search(self, text: &str, flags: u32) -> Option<Match> {

    }
}
