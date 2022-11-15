/// Contains the various error types the application can produce.
#[derive(Debug, Clone)]
pub enum Error {
    /// Syntax error: the referenced string could not be parsed as a valid regex pattern.
    Syntax(&'static str),
    /// Argument error: a function was called with an invalid argument.
    Argument(&'static str),
}

/// Contains the start and end offsets of a pattern match.
#[derive(Debug)]
pub struct Match {
    /// The start of the match (inclusive).
    start: isize,
    /// The end of the match (exclusive).
    end: isize,
}

impl Match {
    /// Creates a new match instance with the given start and end.
    pub fn new(start: isize, end: isize) -> Self {
        return Match { start, end }
    }

    /// Creates a new match instance from unsized indices.
    pub fn from(start: usize, end: usize) -> Self {
        return Match { start: start as isize, end: end as isize }
    }

    /// Returns the start of the match (inclusive).
    pub fn start(&self) -> isize {
        return self.start;
    }

    /// Returns the end of the match (exclusive).
    pub fn end(&self) -> isize {
        return self.end;
    }
}
