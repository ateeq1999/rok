use rok_config::{Config, ConfigFormat};
use std::collections::HashMap;

// ── builder from defaults ────────────────────────────────────────────────────

#[test]
fn defaults_are_accessible() {
    let mut defaults = HashMap::new();
    defaults.insert("app.name".into(), "rok".into());
    defaults.insert("app.version".into(), "1".into());

    let cfg = Config::builder().defaults(defaults).build().unwrap();

    assert_eq!(cfg.get_str("app.name"), Some("rok"));
    let v: u32 = cfg.get("app.version").unwrap();
    assert_eq!(v, 1);
}

// ── JSON file loading ─────────────────────────────────────────────────────────

#[test]
fn loads_json_file() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("config.json");
    std::fs::write(
        &path,
        r#"{"server": {"port": 8080, "host": "localhost"}}"#,
    )
    .unwrap();

    let cfg = Config::builder()
        .file(&path, ConfigFormat::Json)
        .build()
        .unwrap();

    assert_eq!(cfg.get_str("server.host"), Some("localhost"));
    let port: u16 = cfg.get("server.port").unwrap();
    assert_eq!(port, 8080);
}

// ── TOML file loading ─────────────────────────────────────────────────────────

#[test]
fn loads_toml_file() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("config.toml");
    std::fs::write(
        &path,
        "[database]\nurl = \"postgres://localhost/rok\"\npool = 5\n",
    )
    .unwrap();

    let cfg = Config::builder()
        .file(&path, ConfigFormat::Toml)
        .build()
        .unwrap();

    assert_eq!(
        cfg.get_str("database.url"),
        Some("postgres://localhost/rok")
    );
    let pool: u8 = cfg.get("database.pool").unwrap();
    assert_eq!(pool, 5);
}

// ── YAML file loading ─────────────────────────────────────────────────────────

#[test]
fn loads_yaml_file() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("config.yaml");
    std::fs::write(&path, "feature:\n  dark_mode: true\n").unwrap();

    let cfg = Config::builder()
        .file(&path, ConfigFormat::Yaml)
        .build()
        .unwrap();

    assert_eq!(cfg.get_str("feature.dark_mode"), Some("true"));
}

// ── env merging ───────────────────────────────────────────────────────────────

#[test]
fn env_overrides_file() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("base.toml");
    std::fs::write(&path, "[server]\nport = 3000\n").unwrap();

    std::env::set_var("_ROKTEST_SERVER__PORT", "9999");

    let cfg = Config::builder()
        .file(&path, ConfigFormat::Toml)
        .env("_ROKTEST_")
        .build()
        .unwrap();

    std::env::remove_var("_ROKTEST_SERVER__PORT");

    let port: u16 = cfg.get("server.port").unwrap();
    assert_eq!(port, 9999);
}

// ── missing key error ─────────────────────────────────────────────────────────

#[test]
fn missing_key_returns_error() {
    let cfg = Config::builder().build().unwrap();
    assert!(cfg.get::<String>("nonexistent").is_err());
    assert!(!cfg.contains("nonexistent"));
}
