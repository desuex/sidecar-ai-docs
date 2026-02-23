use rusqlite::Connection;
use sidecar_types::SidecarError;

use crate::schema;

/// Run all migrations to bring the database to the latest schema.
pub fn migrate_to_latest(conn: &Connection) -> Result<(), SidecarError> {
    conn.execute_batch(schema::CREATE_META)
        .map_err(|e| SidecarError::Index(e.to_string()))?;

    // Check current schema version
    let current_version: Option<String> = conn
        .query_row(
            "SELECT value FROM meta WHERE key = 'schema_version'",
            [],
            |row| row.get(0),
        )
        .ok();

    let needs_init = current_version.is_none();

    if needs_init {
        conn.execute_batch(schema::CREATE_FILES)
            .map_err(|e| SidecarError::Index(e.to_string()))?;
        conn.execute_batch(schema::CREATE_SYMBOLS)
            .map_err(|e| SidecarError::Index(e.to_string()))?;
        conn.execute_batch(schema::CREATE_REFS)
            .map_err(|e| SidecarError::Index(e.to_string()))?;
        conn.execute_batch(schema::CREATE_DOCS)
            .map_err(|e| SidecarError::Index(e.to_string()))?;

        conn.execute(
            "INSERT INTO meta (key, value) VALUES ('schema_version', ?1)",
            [schema::SCHEMA_VERSION.to_string()],
        )
        .map_err(|e| SidecarError::Index(e.to_string()))?;

        conn.execute(
            "INSERT INTO meta (key, value) VALUES ('uid_format_version', ?1)",
            [schema::UID_FORMAT_VERSION.to_string()],
        )
        .map_err(|e| SidecarError::Index(e.to_string()))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migrate_creates_tables() {
        let conn = Connection::open_in_memory().unwrap();
        migrate_to_latest(&conn).unwrap();

        // Verify tables exist
        let tables: Vec<String> = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")
            .unwrap()
            .query_map([], |row| row.get(0))
            .unwrap()
            .filter_map(|r| r.ok())
            .collect();

        assert!(tables.contains(&"meta".to_string()));
        assert!(tables.contains(&"files".to_string()));
        assert!(tables.contains(&"symbols".to_string()));
        assert!(tables.contains(&"refs".to_string()));
        assert!(tables.contains(&"docs".to_string()));
    }

    #[test]
    fn migrate_is_idempotent() {
        let conn = Connection::open_in_memory().unwrap();
        migrate_to_latest(&conn).unwrap();
        migrate_to_latest(&conn).unwrap(); // should not fail
    }

    #[test]
    fn schema_version_stored() {
        let conn = Connection::open_in_memory().unwrap();
        migrate_to_latest(&conn).unwrap();

        let version: String = conn
            .query_row(
                "SELECT value FROM meta WHERE key = 'schema_version'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(version, "1");
    }
}
