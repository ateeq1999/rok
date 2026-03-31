//! Cache management for rok
//!
//! Provides cache statistics and management operations

use std::fs;
use std::path::{Path, PathBuf};
use serde::Serialize;

/// Cache statistics
#[derive(Debug, Serialize)]
pub struct CacheStats {
    pub enabled: bool,
    pub cache_dir: String,
    pub total_entries: usize,
    pub total_size_bytes: u64,
    pub total_size_human: String,
    pub oldest_entry: Option<String>,
    pub newest_entry: Option<String>,
}

/// Get cache statistics
pub fn get_stats(cache_dir: &Path, enabled: bool) -> CacheStats {
    let mut total_entries = 0;
    let mut total_size_bytes = 0u64;
    let mut oldest_entry: Option<String> = None;
    let mut newest_entry: Option<String> = None;

    if enabled && cache_dir.exists() {
        if let Ok(entries) = fs::read_dir(cache_dir) {
            for entry in entries.flatten() {
                if entry.path().extension().map_or(false, |ext| ext == "json") {
                    total_entries += 1;

                    if let Ok(metadata) = entry.metadata() {
                        total_size_bytes += metadata.len();

                        if let Ok(modified) = metadata.modified() {
                            let timestamp = modified
                                .duration_since(std::time::UNIX_EPOCH)
                                .map(|d| d.as_secs())
                                .unwrap_or(0);

                            let date = chrono::DateTime::from_timestamp(timestamp as i64, 0)
                                .map(|dt| dt.to_rfc3339())
                                .unwrap_or_default();

                            if oldest_entry.is_none() || date < *oldest_entry.as_ref().unwrap() {
                                oldest_entry = Some(date.clone());
                            }
                            if newest_entry.is_none() || date > *newest_entry.as_ref().unwrap() {
                                newest_entry = Some(date);
                            }
                        }
                    }
                }
            }
        }
    }

    CacheStats {
        enabled,
        cache_dir: cache_dir.to_string_lossy().to_string(),
        total_entries,
        total_size_bytes,
        total_size_human: format_bytes(total_size_bytes),
        oldest_entry,
        newest_entry,
    }
}

/// Clear cache
pub fn clear(cache_dir: &Path) -> Result<usize, String> {
    if !cache_dir.exists() {
        return Ok(0);
    }

    let mut cleared = 0;
    if let Ok(entries) = fs::read_dir(cache_dir) {
        for entry in entries.flatten() {
            if entry.path().extension().map_or(false, |ext| ext == "json") {
                if fs::remove_file(entry.path()).is_ok() {
                    cleared += 1;
                }
            }
        }
    }

    Ok(cleared)
}

/// Format bytes to human-readable string
fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Get default cache directory
pub fn get_default_cache_dir() -> PathBuf {
    PathBuf::from(".rok/cache")
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(100), "100 B");
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(1048576), "1.00 MB");
        assert_eq!(format_bytes(1073741824), "1.00 GB");
    }

    #[test]
    fn test_get_stats_empty_cache() {
        let temp_dir = TempDir::new().unwrap();
        let stats = get_stats(temp_dir.path(), true);
        assert_eq!(stats.total_entries, 0);
        assert_eq!(stats.total_size_bytes, 0);
    }

    #[test]
    fn test_get_stats_disabled() {
        let temp_dir = TempDir::new().unwrap();
        let stats = get_stats(temp_dir.path(), false);
        assert!(!stats.enabled);
        assert_eq!(stats.total_entries, 0);
    }

    #[test]
    fn test_clear_cache() {
        let temp_dir = TempDir::new().unwrap();
        // Create some cache files
        fs::write(temp_dir.path().join("cache1.json"), "{}").unwrap();
        fs::write(temp_dir.path().join("cache2.json"), "{}").unwrap();
        fs::write(temp_dir.path().join("other.txt"), "test").unwrap();

        let cleared = clear(temp_dir.path()).unwrap();
        assert_eq!(cleared, 2); // Only .json files

        // Verify non-json file remains
        assert!(temp_dir.path().join("other.txt").exists());
    }
}
