use std::{fmt, path::PathBuf};

#[derive(Debug)]
pub enum AppError {
    FileRead {
        path: PathBuf,
        cause: std::io::Error,
    },
    Parse {
        path: PathBuf,
        line: usize,
        message: &'static str,
    },
    InvalidArgs(&'static str),
}
// Display controls what the user sees.
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Parse {
                path,
                line,
                message,
            } => write!(f, "{}, {}:{}", message, path.display(), line),
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
