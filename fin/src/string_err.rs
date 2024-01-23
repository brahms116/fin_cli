use std::error::Error;
use std::fmt::{Debug, Display};

#[derive(Debug)]
pub struct StringErr(String);

impl StringErr {
    pub fn new(s: String) -> Self {
        Self(s)
    }
}

impl Display for StringErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for StringErr {}
