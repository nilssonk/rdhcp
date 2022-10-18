use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct InvalidLength {
    expected: usize,
    actual: usize,
}

impl InvalidLength {
    #[inline(always)]
    pub(crate) fn new(expected: usize, actual: usize) -> Self {
        Self { expected, actual }
    }
}

impl fmt::Display for InvalidLength {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "InvalidLength{{expected: {}, actual: {}}}",
            self.expected, self.actual
        )
    }
}

impl std::error::Error for InvalidLength {}
