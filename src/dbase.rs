use std::path::{Path, PathBuf};
use std::time::SystemTime;

use anyhow::{Context, Result};
use log::debug;
use tokio::fs;

use crate::types::Bookmark;

/// Database contents.
pub type Content = Vec<Bookmark>;

/// Bookmark database.
#[derive(Debug)]
pub struct Database {
    /// Database file.
    path: PathBuf,
    /// Database contents.
    data: Content,
    /// Sync timestamp.
    sync: SystemTime,
    /// Dirty status flag.
    used: bool,
}

impl Database {
    /// Opens a database from a file.
    pub async fn open(path: PathBuf) -> Result<Self> {
        // Construct database from parts
        Ok(Database {
            used: false,
            sync: SystemTime::now(),
            data: Self::load(&path).await?,
            path,
        })
    }

    /// Gets a read-only view of the database contents.
    pub fn view(&self) -> &[Bookmark] {
        &self.data
    }

    // Pushes an item onto the database.
    pub fn push(&mut self, item: Bookmark) {
        self.data.push(item);
        self.used = true;
    }

    /// Synchronizes with the file on disk.
    pub async fn sync(&mut self) -> Result<()> {
        // Check if file has been modified
        let modified = fs::metadata(&self.path)
            .await
            .context("failed to read metadata")?
            .modified()
            .map_or(true, |time| self.sync < time);

        // Read database from disk (if modified since last sync)
        if modified {
            // TODO: Sync with internal
            let _read = Self::load(&self.path).await?;
        }

        // Write database to disk (if modified internally)
        if self.used {
            Self::dump(&self.path, &self.data).await?;
            self.sync = SystemTime::now();
            self.used = false;
        }

        Ok(())
    }

    /// Load database from file.
    async fn load(path: impl AsRef<Path>) -> Result<Content> {
        // Read database from file
        debug!("reading: `{}`", path.as_ref().display());
        let text = fs::read_to_string(path)
            .await
            .context("failed to read database")?;
        // Parse database from text
        serde_json::from_str(&text).context("failed to parse database")
    }

    /// Dump database to file.
    async fn dump(path: impl AsRef<Path>, data: &Content) -> Result<()> {
        // Serialize database to text
        let text = serde_json::to_string_pretty(data).context("failed to serialize database")?;
        // Write text to file
        debug!("writing: `{}`", path.as_ref().display());
        fs::write(path, text)
            .await
            .context("failed to write database")?;
        Ok(())
    }
}
