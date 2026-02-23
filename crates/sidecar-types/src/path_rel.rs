use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Repo-relative path. Rejects `../` traversal, backslashes, and absolute paths.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PathRel(String);

impl PathRel {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for PathRel {
    type Err = PathRelError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(PathRelError::Empty);
        }
        if s.starts_with('/') || s.starts_with('\\') {
            return Err(PathRelError::Absolute);
        }
        if s.contains('\\') {
            return Err(PathRelError::Backslash);
        }
        // Check for traversal in any segment
        for segment in s.split('/') {
            if segment == ".." {
                return Err(PathRelError::Traversal);
            }
        }
        Ok(PathRel(s.to_owned()))
    }
}

impl fmt::Display for PathRel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum PathRelError {
    #[error("path is empty")]
    Empty,
    #[error("path is absolute")]
    Absolute,
    #[error("path contains backslash")]
    Backslash,
    #[error("path contains traversal (..)")]
    Traversal,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_paths() {
        assert!("src/main.rs".parse::<PathRel>().is_ok());
        assert!("a/b/c.ts".parse::<PathRel>().is_ok());
        assert!("file.txt".parse::<PathRel>().is_ok());
    }

    #[test]
    fn rejects_empty() {
        assert!("".parse::<PathRel>().is_err());
    }

    #[test]
    fn rejects_absolute() {
        assert!("/etc/passwd".parse::<PathRel>().is_err());
    }

    #[test]
    fn rejects_backslash() {
        assert!("src\\main.rs".parse::<PathRel>().is_err());
    }

    #[test]
    fn rejects_traversal() {
        assert!("../escape".parse::<PathRel>().is_err());
        assert!("a/../b".parse::<PathRel>().is_err());
    }
}
