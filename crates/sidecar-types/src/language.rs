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

#[cfg(test)]
mod tests {
    use super::Language;

    #[test]
    fn language_code_and_display() {
        assert_eq!(Language::TypeScript.code(), "ts");
        assert_eq!(Language::JavaScript.code(), "js");
        assert_eq!(Language::Rust.code(), "rs");

        assert_eq!(Language::TypeScript.to_string(), "ts");
        assert_eq!(Language::JavaScript.to_string(), "js");
        assert_eq!(Language::Rust.to_string(), "rs");
    }

    #[test]
    fn serde_roundtrip() {
        let encoded = serde_json::to_string(&Language::TypeScript).unwrap();
        assert_eq!(encoded, "\"typescript\"");
        let decoded: Language = serde_json::from_str(&encoded).unwrap();
        assert_eq!(decoded, Language::TypeScript);
    }
}
