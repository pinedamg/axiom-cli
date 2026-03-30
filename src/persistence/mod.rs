use rusqlite::{params, Connection};
use std::path::Path;

pub mod analytics;
pub use analytics::{TokenSavings, AnalyticsProvider};

pub struct PersistenceManager {
    conn: Connection,
}

impl AnalyticsProvider for PersistenceManager {
    fn record_savings(&self, savings: TokenSavings) -> anyhow::Result<()> {
        self.log_saving(&savings.command, savings.raw_bytes, savings.processed_bytes)
    }

    fn get_total_savings(&self) -> anyhow::Result<(usize, usize)> {
        self.get_total_savings()
    }
}

impl PersistenceManager {
    /// Initializes persistence with a specific path
    pub fn new_with_path(db_path: &Path) -> anyhow::Result<Self> {
        let conn = Connection::open(db_path)?;

        // Performance optimizations for CLI startup
        conn.execute_batch("
            PRAGMA journal_mode = WAL;
            PRAGMA synchronous = NORMAL;
            PRAGMA temp_store = MEMORY;
            PRAGMA cache_size = -2000;
        ")?;

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

        conn.execute(
            "CREATE TABLE IF NOT EXISTS session_settings (
                session_id TEXT PRIMARY KEY,
                intelligence_mode TEXT,
                last_updated DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        Ok(Self { conn })
    }

    pub fn set_session_intelligence(&self, session_id: &str, mode: &str) -> anyhow::Result<()> {
        self.conn.execute(
            "INSERT INTO session_settings (session_id, intelligence_mode) 
             VALUES (?1, ?2) 
             ON CONFLICT(session_id) DO UPDATE SET 
                intelligence_mode = excluded.intelligence_mode,
                last_updated = CURRENT_TIMESTAMP",
            params![session_id, mode],
        )?;
        Ok(())
    }

    pub fn get_session_intelligence(&self, session_id: &str) -> anyhow::Result<Option<String>> {
        let mut stmt = self.conn.prepare("SELECT intelligence_mode FROM session_settings WHERE session_id = ?")?;
        let mut rows = stmt.query(params![session_id])?;
        
        if let Some(row) = rows.next()? {
            Ok(Some(row.get(0)?))
        } else {
            Ok(None)
        }
    }

    /// Deletes a specific template
    pub fn delete_template(&self, template: &str) -> anyhow::Result<()> {
        self.conn.execute("DELETE FROM learned_templates WHERE template = ?", params![template])?;
        Ok(())
    }

    /// Clears all learned templates
    pub fn clear_templates(&self) -> anyhow::Result<()> {
        self.conn.execute("DELETE FROM learned_templates", [])?;
        Ok(())
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

    /// Returns the last N saving events
    /// Returns the last N saving events
    pub fn get_recent_history(&self, limit: usize) -> anyhow::Result<Vec<(String, usize, usize)>> {
        let mut stmt = self.conn.prepare(
            "SELECT command, original_size, compressed_size FROM savings_log ORDER BY id DESC LIMIT ?"
        )?;
        let rows = stmt.query_map(params![limit as i64], |row| {
            Ok((row.get(0)?, row.get::<_, i64>(1)? as usize, row.get::<_, i64>(2)? as usize))
        })?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }
}
