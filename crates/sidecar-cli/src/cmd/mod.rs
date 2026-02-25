pub mod doc;
pub mod index;
pub mod mcp;
pub mod refs;
pub mod search;
pub mod symbol;

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::{Path, PathBuf};

    use sidecar_core::model::DocRecord;
    use sidecar_core::Repository;
    use sidecar_storage::SqliteRepository;
    use tempfile::TempDir;

    use super::{doc, index, mcp, refs, search, symbol};

    fn fixture_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/ts-sample")
            .canonicalize()
            .expect("fixture root must exist")
    }

    fn setup_project_copy() -> TempDir {
        let temp = tempfile::Builder::new()
            .prefix("sidecar-cli-tests-")
            .tempdir_in(std::env::temp_dir())
            .expect("tempdir");
        let src_dst = temp.path().join("src");
        fs::create_dir_all(&src_dst).expect("create src dir");

        for filename in ["cart.ts", "types.ts", "utils.ts"] {
            fs::copy(
                fixture_root().join("src").join(filename),
                src_dst.join(filename),
            )
            .expect("copy fixture source");
        }
        temp
    }

    fn index_project(root: &Path, sidecar_dir: &str) {
        assert_eq!(index::run(root.to_str().unwrap(), sidecar_dir, true), 0);
    }

    #[test]
    fn index_success_and_db_open_failure_paths() {
        let temp = setup_project_copy();
        assert_eq!(
            index::run(temp.path().to_str().unwrap(), ".sidecar", true),
            0
        );
        assert_eq!(
            index::run(temp.path().to_str().unwrap(), ".sidecar-alt", false),
            0
        );

        let bad = setup_project_copy();
        let db_path = bad.path().join(".sidecar").join("index.sqlite");
        fs::create_dir_all(&db_path).expect("create db path as dir");
        assert_eq!(
            index::run(bad.path().to_str().unwrap(), ".sidecar", true),
            3
        );
        assert_eq!(
            index::run(bad.path().to_str().unwrap(), ".sidecar", false),
            3
        );
    }

    #[test]
    fn index_fails_when_sidecar_dir_cannot_be_created() {
        let temp = tempfile::tempdir().expect("tempdir");
        let root_file = temp.path().join("root.txt");
        fs::write(&root_file, "not a directory").expect("write root file");

        let code = index::run(root_file.to_str().unwrap(), ".sidecar", true);
        assert_eq!(code, 5);
        let code = index::run(root_file.to_str().unwrap(), ".sidecar", false);
        assert_eq!(code, 5);
    }

    #[test]
    fn search_handles_invalid_limit_missing_db_and_success() {
        let temp = setup_project_copy();
        let root = temp.path().to_str().unwrap();

        assert_eq!(search::run(root, ".sidecar", "CartService", 0, 0, true), 2);
        assert_eq!(search::run(root, ".sidecar", "CartService", 20, 0, true), 3);
        assert_eq!(search::run(root, ".sidecar", "CartService", 0, 0, false), 2);
        assert_eq!(
            search::run(root, ".sidecar", "CartService", 20, 0, false),
            3
        );

        index_project(temp.path(), ".sidecar");
        assert_eq!(search::run(root, ".sidecar", "CartService", 20, 0, true), 0);
        assert_eq!(search::run(root, ".sidecar", "CartService", 1, 0, false), 0);
        assert_eq!(search::run(root, ".sidecar", "NoMatch", 20, 0, false), 0);
    }

    #[test]
    fn symbol_handles_invalid_uid_not_found_and_found() {
        let temp = setup_project_copy();
        let root = temp.path().to_str().unwrap();

        assert_eq!(symbol::run(root, ".sidecar", "bad uid", true), 2);
        assert_eq!(symbol::run(root, ".sidecar", "bad uid", false), 2);
        assert_eq!(
            symbol::run(
                root,
                ".sidecar",
                "sym:ts:src/cart:CartService:866eb7ea",
                true
            ),
            3
        );
        assert_eq!(
            symbol::run(
                root,
                ".sidecar",
                "sym:ts:src/cart:CartService:866eb7ea",
                false
            ),
            3
        );

        index_project(temp.path(), ".sidecar");
        assert_eq!(
            symbol::run(
                root,
                ".sidecar",
                "sym:ts:src/cart:CartService:866eb7ea",
                true
            ),
            0
        );
        assert_eq!(
            symbol::run(
                root,
                ".sidecar",
                "sym:ts:src/cart:CartService:866eb7ea",
                false
            ),
            0
        );
        assert_eq!(
            symbol::run(root, ".sidecar", "sym:ts:src/cart:Missing:abcd1234", true),
            1
        );
        assert_eq!(
            symbol::run(root, ".sidecar", "sym:ts:src/cart:Missing:abcd1234", false),
            1
        );
    }

    #[test]
    fn refs_handles_input_validation_and_success_paths() {
        let temp = setup_project_copy();
        let root = temp.path().to_str().unwrap();
        let uid = "sym:ts:src/cart:CartService:866eb7ea";

        assert_eq!(refs::run(root, ".sidecar", uid, 0, 0, true), 2);
        assert_eq!(refs::run(root, ".sidecar", "bad uid", 20, 0, true), 2);
        assert_eq!(refs::run(root, ".sidecar", uid, 20, 0, true), 1);
        assert_eq!(refs::run(root, ".sidecar", uid, 0, 0, false), 2);
        assert_eq!(refs::run(root, ".sidecar", "bad uid", 20, 0, false), 2);
        assert_eq!(refs::run(root, ".sidecar", uid, 20, 0, false), 1);

        index_project(temp.path(), ".sidecar");
        assert_eq!(refs::run(root, ".sidecar", uid, 20, 0, true), 0);
        assert_eq!(refs::run(root, ".sidecar", uid, 1, 0, false), 0);
        assert_eq!(
            refs::run(
                root,
                ".sidecar",
                "sym:ts:src/cart:Missing:abcd1234",
                20,
                0,
                false
            ),
            0
        );
    }

    fn seed_doc(root: &Path, sidecar_dir: &str, rel_path: &str) {
        let db_path = root.join(sidecar_dir).join("index.sqlite");
        let repo = SqliteRepository::open(&db_path).expect("open db");
        let doc = DocRecord {
            doc_uid: "doc:cart-overview".parse().unwrap(),
            target_uid: "sym:ts:src/cart:CartService:866eb7ea".parse().unwrap(),
            path: rel_path.parse().unwrap(),
            summary_cache: Some("seed summary".to_owned()),
            updated_at: "2026-01-01T00:00:00Z".to_owned(),
        };
        repo.upsert_docs(&[doc]).expect("upsert doc");
    }

    #[test]
    fn doc_handles_error_paths_and_success_modes() {
        let temp = setup_project_copy();
        let root = temp.path().to_str().unwrap();
        let sidecar_dir = ".sidecar";
        let target_uid = "sym:ts:src/cart:CartService:866eb7ea";

        assert_eq!(doc::run(root, sidecar_dir, "bad uid", "summary", true), 2);
        assert_eq!(doc::run(root, sidecar_dir, target_uid, "summary", true), 3);
        assert_eq!(doc::run(root, sidecar_dir, "bad uid", "summary", false), 2);
        assert_eq!(doc::run(root, sidecar_dir, target_uid, "summary", false), 3);

        index_project(temp.path(), sidecar_dir);
        assert_eq!(doc::run(root, sidecar_dir, target_uid, "summary", true), 1);
        assert_eq!(doc::run(root, sidecar_dir, target_uid, "summary", false), 1);

        seed_doc(temp.path(), sidecar_dir, "docs-sidecar/missing.md");
        assert_eq!(doc::run(root, sidecar_dir, target_uid, "summary", true), 5);
        assert_eq!(doc::run(root, sidecar_dir, target_uid, "summary", false), 5);

        let docs_dir = temp.path().join("docs-sidecar");
        fs::create_dir_all(&docs_dir).expect("create docs dir");
        fs::write(docs_dir.join("missing.md"), "not-front-matter").expect("write bad doc");
        assert_eq!(doc::run(root, sidecar_dir, target_uid, "summary", true), 5);
        assert_eq!(doc::run(root, sidecar_dir, target_uid, "summary", false), 5);

        fs::write(
            docs_dir.join("missing.md"),
            r#"---
doc_uid: doc:cart-overview
title: Cart Overview
anchors:
  - anchor_type: symbol
    symbol_uid: sym:ts:src/cart:CartService:866eb7ea
---

## Overview

This is a deterministic summary.
"#,
        )
        .expect("write valid doc");

        assert_eq!(doc::run(root, sidecar_dir, target_uid, "summary", true), 0);
        assert_eq!(doc::run(root, sidecar_dir, target_uid, "full", true), 0);
        assert_eq!(doc::run(root, sidecar_dir, target_uid, "summary", false), 0);
        assert_eq!(doc::run(root, sidecar_dir, target_uid, "full", false), 0);

        // Front-matter-only docs exercise the no-summary human output branch.
        fs::write(
            docs_dir.join("missing.md"),
            r#"---
doc_uid: doc:cart-overview
title: Cart Overview
anchors:
  - anchor_type: symbol
    symbol_uid: sym:ts:src/cart:CartService:866eb7ea
---
"#,
        )
        .expect("write summaryless doc");
        assert_eq!(doc::run(root, sidecar_dir, target_uid, "summary", false), 0);
    }

    #[test]
    fn mcp_returns_error_when_sidecar_dir_is_invalid() {
        let temp = tempfile::tempdir().expect("tempdir");
        let root_file = temp.path().join("root.txt");
        fs::write(&root_file, "x").expect("write root file");

        let code = mcp::run(root_file.to_str().unwrap(), ".sidecar");
        assert_eq!(code, 5);
    }
}
