use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct ExecutionFailed {
    stderr: String,
    code: Option<i32>,
}

impl ExecutionFailed {
    pub fn new(stderr: String, code: Option<i32>) -> Self {
        ExecutionFailed {
            stderr,
            code,
        }
    }
}

impl Display for ExecutionFailed {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.code {
            Some(code) => write!(f, "Command failed with Code '{}' | Stderr:\n{}", code, self.stderr),
            None => write!(f, "Command failed | Stderr:\n{}", self.stderr),
        }
    }
}

impl Error for ExecutionFailed {}