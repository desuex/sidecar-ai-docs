use std::path::Path;
use std::{fs, io};

use sidecar_mcp::server::McpServer;
use sidecar_storage::SqliteRepository;

pub fn run(root: &str, sidecar_dir: &str) -> i32 {
    run_with(root, sidecar_dir, |server| server.run())
}

fn run_with<F>(root: &str, sidecar_dir: &str, serve: F) -> i32
where
    F: FnOnce(&McpServer<SqliteRepository>) -> Result<(), io::Error>,
{
    let db_dir = Path::new(root).join(sidecar_dir);
    if let Err(e) = fs::create_dir_all(&db_dir) {
        eprintln!("sidecar mcp: cannot create sidecar directory: {e}");
        return 5;
    }

    let db_path = db_dir.join("index.sqlite");
    let repo = match SqliteRepository::open(&db_path) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("sidecar mcp: {e}");
            return 3;
        }
    };

    let server = McpServer::new(repo, Path::new(root));
    match serve(&server) {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("sidecar mcp: server I/O error: {e}");
            5
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, io};

    use super::run_with;

    #[test]
    fn returns_db_open_error_when_index_path_is_directory() {
        let temp = tempfile::tempdir().unwrap();
        let db_dir = temp.path().join(".sidecar");
        fs::create_dir_all(db_dir.join("index.sqlite")).unwrap();

        let code = run_with(temp.path().to_str().unwrap(), ".sidecar", |_| Ok(()));
        assert_eq!(code, 3);
    }

    #[test]
    fn returns_success_when_server_exits_cleanly() {
        let temp = tempfile::tempdir().unwrap();

        let code = run_with(temp.path().to_str().unwrap(), ".sidecar", |_| Ok(()));
        assert_eq!(code, 0);
    }

    #[test]
    fn returns_internal_error_on_server_io_failure() {
        let temp = tempfile::tempdir().unwrap();

        let code = run_with(temp.path().to_str().unwrap(), ".sidecar", |_| {
            Err(io::Error::other("forced failure"))
        });
        assert_eq!(code, 5);
    }
}
