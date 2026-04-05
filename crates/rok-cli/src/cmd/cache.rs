use crate::cache;

pub fn manage(stats: bool, clear: bool) -> i32 {
    let cache_dir = std::path::Path::new(".rok/cache");

    if clear {
        match cache::clear(cache_dir) {
            Ok(count) => {
                println!("Cleared {} cache entries", count);
                return 0;
            }
            Err(e) => {
                eprintln!("Error clearing cache: {}", e);
                return 1;
            }
        }
    }

    if stats || !clear {
        let cache_stats = cache::get_stats(cache_dir, true);
        println!("Cache Statistics:");
        println!("  Enabled: {}", cache_stats.enabled);
        println!("  Directory: {}", cache_stats.cache_dir);
        println!("  Total Entries: {}", cache_stats.total_entries);
        println!(
            "  Total Size: {} ({})",
            cache_stats.total_size_human, cache_stats.total_size_bytes
        );
        if let Some(oldest) = &cache_stats.oldest_entry {
            println!("  Oldest Entry: {}", oldest);
        }
        if let Some(newest) = &cache_stats.newest_entry {
            println!("  Newest Entry: {}", newest);
        }
        if cache_stats.total_entries == 0 {
            println!("\nCache is empty. Run tasks with --cache to populate.");
        }
    }
    0
}

pub fn checkpoints(list: bool, delete: Option<&str>) -> i32 {
    let checkpoint_dir = std::path::Path::new(".rok/checkpoints");

    if let Some(id) = delete {
        let file = checkpoint_dir.join(format!("{}.json", id));
        if file.exists() {
            std::fs::remove_file(&file).expect("Failed to delete checkpoint");
            println!("Deleted checkpoint: {}", id);
        } else {
            eprintln!("Checkpoint not found: {}", id);
            return 1;
        }
        return 0;
    }

    if list || delete.is_none() {
        if !checkpoint_dir.exists() {
            println!("No checkpoints found.");
            return 0;
        }

        let mut found = false;
        if let Ok(entries) = std::fs::read_dir(checkpoint_dir) {
            for entry in entries.flatten() {
                if entry.path().extension().map(|e| e == "json").unwrap_or(false) {
                    if let Ok(content) = std::fs::read_to_string(entry.path()) {
                        if let Ok(data) = serde_json::from_str::<serde_json::Value>(&content) {
                            if !found {
                                println!("Checkpoints:");
                                found = true;
                            }
                            println!(
                                "  {} - created: {}",
                                data["checkpoint_id"].as_str().unwrap_or("?"),
                                data["created_at"].as_str().unwrap_or("?"),
                            );
                        }
                    }
                }
            }
        }

        if !found {
            println!("No checkpoints found.");
        }
    }
    0
}
