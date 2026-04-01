//! Built-in Tera template strings.

/// A `struct` with optional field list.
pub const MODEL: &str = r#"
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {{ name }} {
{% for field in fields %}    pub {{ field }}: String,
{% endfor %}}
"#;

/// An async Axum handler that returns a JSON list (stub).
pub const HANDLER: &str = r#"
use axum::Json;
use serde_json::Value;

pub async fn list_{{ name | lower }}() -> Json<Vec<Value>> {
    Json(vec![])
}

pub async fn get_{{ name | lower }}() -> Json<Value> {
    Json(serde_json::json!({}))
}
"#;

/// A basic SQL migration file.
pub const MIGRATION: &str = r#"
-- Migration: create_{{ name | lower }}s
-- Created: {{ created_at }}

CREATE TABLE IF NOT EXISTS {{ name | lower }}s (
    id   UUID PRIMARY KEY DEFAULT gen_random_uuid(),
{% for field in fields %}    {{ field }} TEXT NOT NULL,
{% endfor %}    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
"#;

/// A repository struct with basic CRUD stubs.
pub const REPOSITORY: &str = r#"
use anyhow::Result;

pub struct {{ name }}Repository;

impl {{ name }}Repository {
    pub async fn find_all(&self) -> Result<Vec<{{ name }}>> {
        todo!()
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<{{ name }}>> {
        todo!()
    }

    pub async fn create(&self, item: {{ name }}) -> Result<{{ name }}> {
        todo!()
    }

    pub async fn delete(&self, id: &str) -> Result<bool> {
        todo!()
    }
}
"#;
