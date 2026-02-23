/// Print a "not implemented" stub and return exit code.
pub fn not_implemented(command: &str, json: bool) -> i32 {
    if json {
        println!(
            "{}",
            serde_json::json!({
                "error": "not_implemented",
                "command": command
            })
        );
    } else {
        eprintln!("sidecar {command}: not implemented");
    }
    5
}
