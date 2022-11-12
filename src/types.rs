/// Contains the various error types the application can produce.
pub enum Error {
    /// Syntax error: the referenced string could not be parsed as a valid regex pattern.
    Syntax(&'static str),
}

/// Contains the start and end offsets of a pattern match.
#[derive(Debug)]
pub struct Match {
    /// The start of the match (inclusive).
    start: usize,
    /// The end of the match (exclusive).
    end: usize,
}

impl Match {
    /// Creates a new match instance with the given start and end.
    pub fn new(start: usize, end: usize) -> Self {
        return Match { start, end }
    }

    /// Returns the start of the match (inclusive).
    pub fn start(&self) -> usize {
        return self.start;
    }

    /// Returns the end of the match (exclusive).
    pub fn end(&self) -> usize {
        return self.end;
    }
}
