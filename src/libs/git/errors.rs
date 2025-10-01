use std::io;

#[derive(Debug)]
pub enum GitError {
    CommandFailed(String),
    IoError(io::Error),
    TagExists(String),
}

impl From<io::Error> for GitError {
    fn from(err: io::Error) -> Self {
        GitError::IoError(err)
    }
}

impl std::fmt::Display for GitError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GitError::CommandFailed(e) => write!(f, "Git command failed: {}", e),
            GitError::IoError(e) => write!(f, "IO error: {}", e),
            GitError::TagExists(tag) => write!(f, "Tag '{}' already exists", tag),
        }
    }
}

impl std::error::Error for GitError {}
