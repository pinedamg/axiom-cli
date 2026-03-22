use rusqlite::{params, Connection};
use std::path::Path;

pub struct PersistenceManager {
    conn: Connection,
}

impl PersistenceManager {
    /// Initializes persistence with a specific path
    pub fn new_with_path(db_path: &Path) -> anyhow::Result<Self> {
        let conn = Connection::open(db_path)?;

        // Create base tables
        conn.execute(
            "CREATE TABLE IF NOT EXISTS learned_templates (
                id INTEGER PRIMARY KEY,
                template TEXT UNIQUE,
                frequency INTEGER DEFAULT 0,
                last_seen DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS savings_log (
                id INTEGER PRIMARY KEY,
                command TEXT,
                original_size INTEGER,
                compressed_size INTEGER,
                timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        Ok(Self { conn })
    }

    pub fn new() -> anyhow::Result<Self> {
        Self::new_with_path(Path::new("axiom.db"))
    }

    /// Saves or updates a learned template
    pub fn upsert_template(&self, template: &str, frequency: usize) -> anyhow::Result<()> {
        self.conn.execute(
            "INSERT INTO learned_templates (template, frequency) 
             VALUES (?1, ?2) 
             ON CONFLICT(template) DO UPDATE SET 
                frequency = excluded.frequency,
                last_seen = CURRENT_TIMESTAMP",
            params![template, frequency as i64],
        )?;
        Ok(())
    }

    /// Retrieves all known templates
    pub fn get_known_templates(&self) -> anyhow::Result<Vec<(String, usize)>> {
        let mut stmt = self.conn.prepare("SELECT template, frequency FROM learned_templates")?;
        let rows = stmt.query_map([], |row| {
            Ok((row.get(0)?, row.get::<_, i64>(1)? as usize))
        })?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    /// Logs a saving event
    pub fn log_saving(&self, command: &str, original: usize, compressed: usize) -> anyhow::Result<()> {
        self.conn.execute(
            "INSERT INTO savings_log (command, original_size, compressed_size) VALUES (?1, ?2, ?3)",
            params![command, original as i64, compressed as i64],
        )?;
        Ok(())
    }

    /// Returns total characters saved: (original, compressed)
    pub fn get_total_savings(&self) -> anyhow::Result<(usize, usize)> {
        let mut stmt = self.conn.prepare("SELECT SUM(original_size), SUM(compressed_size) FROM savings_log")?;
        let mut rows = stmt.query([])?;
        
        if let Some(row) = rows.next()? {
            let original: i64 = row.get(0).unwrap_or(0);
            let compressed: i64 = row.get(1).unwrap_or(0);
            Ok((original as usize, compressed as usize))
        } else {
            Ok((0, 0))
        }
    }
}
