use serde::{Deserialize, Serialize};

const DEFAULT_LIMIT: u32 = 20;
const MAX_LIMIT: u32 = 1000;

/// Bounded query limit (1..=1000, default 20).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Limit(u32);

impl Limit {
    pub fn new(n: u32) -> Result<Self, LimitError> {
        if n == 0 {
            Err(LimitError::Zero)
        } else if n > MAX_LIMIT {
            Err(LimitError::TooLarge(n))
        } else {
            Ok(Limit(n))
        }
    }

    pub fn value(self) -> u32 {
        self.0
    }
}

impl Default for Limit {
    fn default() -> Self {
        Limit(DEFAULT_LIMIT)
    }
}

/// Pagination offset.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Offset(u32);

impl Offset {
    pub fn new(n: u32) -> Self {
        Offset(n)
    }

    pub fn value(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum LimitError {
    #[error("limit must be at least 1")]
    Zero,
    #[error("limit {0} exceeds maximum {MAX_LIMIT}")]
    TooLarge(u32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_limit() {
        assert_eq!(Limit::default().value(), 20);
    }

    #[test]
    fn valid_limit() {
        assert_eq!(Limit::new(1).unwrap().value(), 1);
        assert_eq!(Limit::new(1000).unwrap().value(), 1000);
    }

    #[test]
    fn rejects_zero() {
        assert!(Limit::new(0).is_err());
    }

    #[test]
    fn rejects_too_large() {
        assert!(Limit::new(1001).is_err());
    }
}
