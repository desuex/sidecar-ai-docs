//! Parser for sidecar documentation files (YAML front matter + Markdown body).

use serde::Deserialize;
use sidecar_types::SidecarError;

/// YAML front matter of a sidecar doc file.
#[derive(Debug, Deserialize)]
pub struct SidecarDocFrontmatter {
    pub doc_uid: String,
    pub title: String,
    #[serde(default)]
    pub anchors: Vec<Anchor>,
    pub updated_at: Option<String>,
}

/// An anchor binding a doc to a code symbol or selector.
#[derive(Debug, Deserialize)]
pub struct Anchor {
    pub anchor_type: String,
    pub symbol_uid: Option<String>,
    pub fingerprint: Option<String>,
    pub confidence: Option<f64>,
}

/// Parse a sidecar doc file into front matter + markdown body.
///
/// Expects the format:
/// ```text
/// ---
/// doc_uid: ...
/// title: ...
/// anchors: [...]
/// ---
/// ## Overview
/// ...
/// ```
pub fn parse_sidecar_doc(content: &str) -> Result<(SidecarDocFrontmatter, String), SidecarError> {
    let content = content.trim();

    // Must start with ---
    if !content.starts_with("---") {
        return Err(SidecarError::Parse(
            "sidecar doc must start with YAML front matter (---)".to_owned(),
        ));
    }

    // Find the closing ---
    let after_first = &content[3..];
    let closing = after_first
        .find("\n---")
        .ok_or_else(|| SidecarError::Parse("missing closing --- in front matter".to_owned()))?;

    let yaml_str = &after_first[..closing];
    let body = after_first[closing + 4..].trim().to_owned();

    let front_matter: SidecarDocFrontmatter = serde_yaml::from_str(yaml_str)
        .map_err(|e| SidecarError::Parse(format!("invalid YAML front matter: {e}")))?;

    Ok((front_matter, body))
}

/// Extract the summary from a markdown body.
/// Looks for the content under `## Overview` heading.
/// If not found, returns the first paragraph.
pub fn extract_summary(body: &str) -> Option<String> {
    let lines: Vec<&str> = body.lines().collect();

    // Look for ## Overview section
    for (i, line) in lines.iter().enumerate() {
        if line.trim() == "## Overview" {
            // Collect lines until next heading or end
            let mut summary_lines = Vec::new();
            for &subsequent in &lines[i + 1..] {
                if subsequent.starts_with("## ") {
                    break;
                }
                summary_lines.push(subsequent);
            }
            let summary = summary_lines.join("\n").trim().to_owned();
            if !summary.is_empty() {
                return Some(summary);
            }
        }
    }

    // Fallback: first non-empty paragraph
    let first_para: String = lines
        .iter()
        .skip_while(|l| l.trim().is_empty())
        .take_while(|l| !l.trim().is_empty())
        .copied()
        .collect::<Vec<_>>()
        .join("\n");

    if first_para.is_empty() {
        None
    } else {
        Some(first_para)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DOC: &str = r#"---
doc_uid: doc:test-uid
title: Test Document
anchors:
  - anchor_type: symbol
    symbol_uid: sym:rs:crates/core/src/lib:Repository:abcd1234
    fingerprint: abc123
    confidence: 1.0
updated_at: "2026-01-01T00:00:00Z"
---

## Overview

This is the summary of the document.
It spans multiple lines.

## Details

This is the detailed section.
"#;

    #[test]
    fn parses_front_matter() {
        let (fm, _body) = parse_sidecar_doc(SAMPLE_DOC).unwrap();
        assert_eq!(fm.doc_uid, "doc:test-uid");
        assert_eq!(fm.title, "Test Document");
        assert_eq!(fm.anchors.len(), 1);
        assert_eq!(fm.anchors[0].anchor_type, "symbol");
        assert_eq!(
            fm.anchors[0].symbol_uid.as_deref(),
            Some("sym:rs:crates/core/src/lib:Repository:abcd1234")
        );
        assert_eq!(fm.anchors[0].confidence, Some(1.0));
    }

    #[test]
    fn extracts_body() {
        let (_fm, body) = parse_sidecar_doc(SAMPLE_DOC).unwrap();
        assert!(body.contains("## Overview"));
        assert!(body.contains("## Details"));
    }

    #[test]
    fn extracts_summary() {
        let (_fm, body) = parse_sidecar_doc(SAMPLE_DOC).unwrap();
        let summary = extract_summary(&body).unwrap();
        assert!(summary.contains("This is the summary of the document."));
        assert!(summary.contains("It spans multiple lines."));
        assert!(!summary.contains("Details"));
    }

    #[test]
    fn rejects_no_front_matter() {
        let result = parse_sidecar_doc("# Just markdown");
        assert!(result.is_err());
    }
}
