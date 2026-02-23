use serde::{Deserialize, Serialize};
use std::fmt;

/// Supported languages for parsing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    TypeScript,
    JavaScript,
    Rust,
}

impl Language {
    /// Short code used in UIDs (e.g., "ts", "js", "rs").
    pub fn code(&self) -> &'static str {
        match self {
            Language::TypeScript => "ts",
            Language::JavaScript => "js",
            Language::Rust => "rs",
        }
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}
