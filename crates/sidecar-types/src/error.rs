/// Top-level error type with stable exit codes.
#[derive(Debug, thiserror::Error)]
pub enum SidecarError {
    #[error("validation error: {0}")]
    Validation(String),

    #[error("index error: {0}")]
    Index(String),

    #[error("parse error: {0}")]
    Parse(String),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("internal error: {0}")]
    Internal(String),
}

impl SidecarError {
    /// Stable exit code for CLI.
    pub fn exit_code(&self) -> i32 {
        match self {
            SidecarError::Validation(_) => 2,
            SidecarError::Index(_) => 3,
            SidecarError::Parse(_) => 4,
            SidecarError::NotFound(_) => 1,
            SidecarError::Internal(_) => 5,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SidecarError;

    #[test]
    fn stable_exit_codes() {
        assert_eq!(SidecarError::Validation("x".to_owned()).exit_code(), 2);
        assert_eq!(SidecarError::Index("x".to_owned()).exit_code(), 3);
        assert_eq!(SidecarError::Parse("x".to_owned()).exit_code(), 4);
        assert_eq!(SidecarError::NotFound("x".to_owned()).exit_code(), 1);
        assert_eq!(SidecarError::Internal("x".to_owned()).exit_code(), 5);
    }
}
