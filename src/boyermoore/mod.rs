use regex::Match;

pub struct BoyerMoore {

}

impl BoyerMoore {
    pub fn compile(pattern: &str, flags: u32) -> Result<Self, String> {
        return Ok(BoyerMoore {});
    }

    pub fn search(self, text: &str, flags: u32) -> Option<Match> {
        return None;
    }
}
