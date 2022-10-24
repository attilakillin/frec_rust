pub enum Error {
    Syntax(&'static str),
}

#[derive(Debug)]
pub struct Match {
    pub start: usize,
    pub end: usize,
}

impl Match {
    pub fn start(&self) -> usize {
        return self.start;
    }

    pub fn end(&self) -> usize {
        return self.end;
    }
}
