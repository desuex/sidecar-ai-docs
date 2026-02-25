mod cmd;

use clap::{Parser, Subcommand};
use tracing_subscriber::EnvFilter;

/// Global options shared across commands that need the index.
#[derive(Parser)]
#[command(name = "sidecar", version, about = "Code documentation infrastructure")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Project root directory
    #[arg(long, global = true, default_value = ".")]
    root: String,

    /// Sidecar directory name (relative to root)
    #[arg(long, global = true, default_value = ".sidecar")]
    sidecar_dir: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Build or update the index
    Index {
        /// Languages to index (comma-separated)
        #[arg(long)]
        languages: Option<String>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Search symbols
    Search {
        /// Search query
        query: String,
        /// Maximum results
        #[arg(long, default_value = "20")]
        limit: u32,
        /// Offset for pagination
        #[arg(long, default_value = "0")]
        offset: u32,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Get symbol details
    Symbol {
        /// Symbol UID
        uid: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Find references to a symbol
    Refs {
        /// Symbol UID
        uid: String,
        /// Maximum results
        #[arg(long, default_value = "20")]
        limit: u32,
        /// Offset for pagination
        #[arg(long, default_value = "0")]
        offset: u32,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Fetch sidecar documentation
    Doc {
        /// Symbol UID
        uid: String,
        /// Output mode
        #[arg(long, default_value = "summary")]
        mode: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Start MCP server on stdio
    Mcp {
        /// Log level
        #[arg(long, default_value = "info")]
        log_level: String,
        /// JSON-formatted logs
        #[arg(long)]
        json_logs: bool,
    },
    /// Export Sidecar docs to external formats
    Export {
        #[command(subcommand)]
        command: ExportCommands,
    },
}

#[derive(Subcommand)]
enum ExportCommands {
    /// Export generated documentation for MkDocs/RTD
    Mkdocs {
        /// Output directory (relative to --root unless absolute)
        #[arg(long, default_value = "docs/generated")]
        out: String,
        /// Optional path to index sqlite DB (relative to --root unless absolute)
        #[arg(long)]
        index_db: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();
    init_logging(&cli);

    let exit_code = run_cli(&cli);
    std::process::exit(exit_code);
}

fn run_cli(cli: &Cli) -> i32 {
    let root = &cli.root;
    let sidecar_dir = &cli.sidecar_dir;

    match cli.command {
        Commands::Index { json, .. } => cmd::index::run(root, sidecar_dir, json),
        Commands::Search {
            ref query,
            limit,
            offset,
            json,
        } => cmd::search::run(root, sidecar_dir, query, limit, offset, json),
        Commands::Symbol { ref uid, json } => cmd::symbol::run(root, sidecar_dir, uid, json),
        Commands::Refs {
            ref uid,
            limit,
            offset,
            json,
        } => cmd::refs::run(root, sidecar_dir, uid, limit, offset, json),
        Commands::Doc {
            ref uid,
            ref mode,
            json,
        } => cmd::doc::run(root, sidecar_dir, uid, mode, json),
        Commands::Mcp { .. } => cmd::mcp::run(root, sidecar_dir),
        Commands::Export { ref command } => match command {
            ExportCommands::Mkdocs { out, index_db } => {
                cmd::export::run_mkdocs(root, out, index_db.as_deref())
            }
        },
    }
}

fn init_logging(cli: &Cli) {
    let (fallback_level, json_logs) = logging_mode(cli);

    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(fallback_level));

    if json_logs {
        let _ = tracing_subscriber::fmt()
            .with_writer(std::io::stderr)
            .with_env_filter(filter)
            .json()
            .try_init();
    } else {
        let _ = tracing_subscriber::fmt()
            .with_writer(std::io::stderr)
            .with_env_filter(filter)
            .try_init();
    }
}

fn logging_mode(cli: &Cli) -> (&str, bool) {
    match &cli.command {
        Commands::Mcp {
            log_level,
            json_logs,
        } => (log_level.as_str(), *json_logs),
        _ => ("info", false),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_search_command_and_logging_defaults() {
        let cli = Cli::try_parse_from(["sidecar", "search", "CartService"]).unwrap();
        assert!(matches!(cli.command, Commands::Search { .. }));
        let (level, json_logs) = logging_mode(&cli);
        assert_eq!(level, "info");
        assert!(!json_logs);
        init_logging(&cli);
    }

    #[test]
    fn parse_mcp_command_and_logging_flags() {
        let cli =
            Cli::try_parse_from(["sidecar", "mcp", "--log-level", "debug", "--json-logs"]).unwrap();
        assert!(matches!(cli.command, Commands::Mcp { .. }));
        let (level, json_logs) = logging_mode(&cli);
        assert_eq!(level, "debug");
        assert!(json_logs);
        init_logging(&cli);
    }

    #[test]
    fn parse_export_mkdocs_command() {
        let cli = Cli::try_parse_from([
            "sidecar",
            "export",
            "mkdocs",
            "--out",
            "docs/generated",
            "--index-db",
            ".sidecar/index.sqlite",
        ])
        .unwrap();
        assert!(matches!(
            cli.command,
            Commands::Export {
                command: ExportCommands::Mkdocs { .. }
            }
        ));
        let (level, json_logs) = logging_mode(&cli);
        assert_eq!(level, "info");
        assert!(!json_logs);
    }

    #[test]
    fn run_cli_covers_command_dispatch() {
        let temp = tempfile::Builder::new()
            .prefix("sidecar-main-tests-")
            .tempdir_in(std::env::temp_dir())
            .unwrap();
        let root_file = temp.path().join("root.txt");
        std::fs::write(&root_file, "x").unwrap();

        let mcp_cli = Cli::try_parse_from([
            "sidecar",
            "--root",
            root_file.to_str().unwrap(),
            "mcp",
            "--log-level",
            "debug",
        ])
        .unwrap();
        assert_eq!(run_cli(&mcp_cli), 5);

        let search_cli =
            Cli::try_parse_from(["sidecar", "--root", ".", "search", "q", "--limit", "0"]).unwrap();
        assert_eq!(run_cli(&search_cli), 2);

        let symbol_cli =
            Cli::try_parse_from(["sidecar", "--root", ".", "symbol", "bad uid"]).unwrap();
        assert_eq!(run_cli(&symbol_cli), 2);

        let index_cli =
            Cli::try_parse_from(["sidecar", "--root", root_file.to_str().unwrap(), "index"])
                .unwrap();
        assert_eq!(run_cli(&index_cli), 5);

        let refs_cli =
            Cli::try_parse_from(["sidecar", "--root", ".", "refs", "bad uid", "--limit", "10"])
                .unwrap();
        assert_eq!(run_cli(&refs_cli), 2);

        let doc_cli = Cli::try_parse_from([
            "sidecar", "--root", ".", "doc", "bad uid", "--mode", "summary",
        ])
        .unwrap();
        assert_eq!(run_cli(&doc_cli), 2);

        let export_cli = Cli::try_parse_from([
            "sidecar",
            "--root",
            root_file.to_str().unwrap(),
            "export",
            "mkdocs",
            "--out",
            "docs/generated",
        ])
        .unwrap();
        assert_eq!(run_cli(&export_cli), 5);
    }
}
