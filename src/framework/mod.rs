
pub enum RegError {
    BadPattern,
    NoMatch,
}

pub struct CompileFlags {
    pub extended: bool,
    pub ignore_case: bool,
}
