/// Represents a pattern match on a given text.
/// 
/// The lifetime parameter `'t` refers to the lifetime of the matched text.
#[derive(Clone, Copy, Debug)]
pub struct Match<'t> {
    /// The starting byte offset of the match (inclusive).
    start: usize,
    /// The ending byte offset of the match (exclusive).
    end: usize,
    /// The text that was matched between the starting and ending offset.
    matched_text: &'t str
}

/// Contains the various error types the application can produce.
#[derive(Clone, Debug)]
pub enum Error {
    /// Syntax error: the referenced string could not be parsed as a valid regex pattern.
    Syntax(&'static str)
}

impl<'t> Match<'t> {
    /// Creates a new match instance with the given start and end.
    pub(crate) fn new(start: usize, end: usize, matched_text: &'t str) -> Self {
        return Match { start, end, matched_text }
    }

    /// Returns the starting byte offset of the match (inclusive).
    pub fn start(&self) -> usize {
        return self.start;
    }

    /// Returns the ending byte offset of the match (exclusive).
    pub fn end(&self) -> usize {
        return self.end;
    }

    /// Returns the matched text.
    pub fn as_str(&self) -> &'t str {
        return self.matched_text;
    }
}
