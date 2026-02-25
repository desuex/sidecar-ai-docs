use std::path::Path;

use sidecar_mcp::server::McpServer;
use sidecar_storage::SqliteRepository;

pub fn run(root: &str, sidecar_dir: &str) -> i32 {
    let db_dir = Path::new(root).join(sidecar_dir);
    if let Err(e) = std::fs::create_dir_all(&db_dir) {
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
    match server.run() {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("sidecar mcp: server I/O error: {e}");
            5
        }
    }
}
