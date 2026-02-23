use serde::{Deserialize, Serialize};

/// Byte offset range within a file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Range {
    pub start: u32,
    pub end: u32,
}
