use std::{fmt, path::PathBuf};

use crate::parser::ParseError;

#[derive(Debug)]
pub enum AppError {
    FileRead {
        path: PathBuf,
        cause: std::io::Error,
    },
    Parse(PathBuf, ParseError),
    InvalidArgs(&'static str),
}

// Display controls what the user sees.
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Parse(path, error) => write!(
                f,
                "Parse error in {} at {}: {}",
                path.display(),
                error.line,
                error.message
            ),
            AppError::FileRead { path, cause } => {
                write!(f, "failed to read file {}: {}", path.display(), cause)
            }
            AppError::InvalidArgs(cause) => {
                write!(f, "invalid args: {}", cause)
            }
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::FileRead { cause, .. } => Some(cause),
            _ => None,
        }
    }
}
