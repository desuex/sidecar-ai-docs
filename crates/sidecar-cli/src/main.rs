mod cmd;
mod output;

use clap::{Parser, Subcommand};

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
}

fn main() {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .init();

    let cli = Cli::parse();
    let root = &cli.root;
    let sidecar_dir = &cli.sidecar_dir;

    let exit_code = match cli.command {
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
        } => cmd::refs::run(uid, limit, offset, json),
        Commands::Doc {
            ref uid,
            ref mode,
            json,
        } => cmd::doc::run(root, sidecar_dir, uid, mode, json),
        Commands::Mcp { .. } => cmd::mcp::run(),
    };

    std::process::exit(exit_code);
}
