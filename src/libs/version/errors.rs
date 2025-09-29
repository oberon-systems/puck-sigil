use std::io;

#[derive(Debug)]
pub enum VersionError {
    IoError(io::Error),
    ParseError(String),
    ParamNotFound(String),
    UnsupportedFormat(String),
}

impl From<io::Error> for VersionError {
    fn from(err: io::Error) -> Self {
        VersionError::IoError(err)
    }
}

impl std::fmt::Display for VersionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            VersionError::IoError(e) => write!(f, "IO error: {}", e),
            VersionError::ParseError(e) => write!(f, "Parse error: {}", e),
            VersionError::ParamNotFound(p) => write!(f, "Parameter '{}' not found", p),
            VersionError::UnsupportedFormat(e) => write!(f, "Unsupported format: {}", e),
        }
    }
}

impl std::error::Error for VersionError {}
