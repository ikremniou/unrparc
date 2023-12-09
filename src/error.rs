use std::{error::Error, fmt, num::ParseIntError};

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

impl From<ParseIntError> for UnrparcError {
    fn from(_error: ParseIntError) -> Self {
        UnrparcError
    }
}

impl From<serde_pickle::Error> for UnrparcError {
    fn from(_error: serde_pickle::Error) -> Self {
        UnrparcError
    }
}
