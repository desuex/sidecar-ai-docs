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
