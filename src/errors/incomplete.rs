use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Incomplete {
    expected: usize,
    actual: usize,
}

impl Incomplete {
    pub(crate) fn new(expected: usize, actual: usize) -> Self {
        Self { expected, actual }
    }
}

impl fmt::Display for Incomplete {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Incomplete{{expected: {}, actual: {}}}",
            self.expected, self.actual
        )
    }
}

impl std::error::Error for Incomplete {}
