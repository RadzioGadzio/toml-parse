use std::{fmt, io::Error, path::PathBuf};

#[derive(Debug)]
pub enum AppError {
    FileRead {
        path: PathBuf,
        source: Error,
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
            AppError::FileRead { path, source } => {
                write!(f, "Failed to read file: {:?} [{}]", source, path.display())
            }
            AppError::InvalidArgs(cause) => {
                write!(f, "Invalid args {}", cause)
            }
        }
    }
}

impl std::error::Error for AppError {}
