use std::{fmt, error::Error};

#[derive(Debug, Clone)]
pub struct UnrparcError;

impl Error for UnrparcError {}

impl fmt::Display for UnrparcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UnrparcError")
    }
}

impl From<std::io::Error> for UnrparcError {
    fn from(_error: std::io::Error) -> Self {
        UnrparcError
    }
}