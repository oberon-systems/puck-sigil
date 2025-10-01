mod errors;

use std::process::Command;

pub use errors::GitError;

pub fn tag_exists(tag: &str) -> Result<bool, GitError> {
    let output = Command::new("git").args(["tag", "-l", tag]).output()?;

    if !output.status.success() {
        return Err(GitError::CommandFailed(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    let result = String::from_utf8_lossy(&output.stdout);
    Ok(!result.trim().is_empty())
}

pub fn create_tag(version: &str) -> Result<(), GitError> {
    let tag = format!("v{}", version);

    if tag_exists(&tag)? {
        return Err(GitError::TagExists(tag));
    }

    let output = Command::new("git").args(["tag", &tag]).output()?;

    if !output.status.success() {
        return Err(GitError::CommandFailed(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests;
