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

/// Full CRUD Axum handlers for a named resource.
pub const CRUD_HANDLER: &str = r#"
use axum::{extract::Path, Json};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct {{ name }}Body {
    // TODO: add fields
}

pub async fn list_{{ name | lower }}() -> Json<Vec<Value>> {
    Json(vec![])
}

pub async fn get_{{ name | lower }}(Path(id): Path<String>) -> Json<Value> {
    Json(serde_json::json!({ "id": id }))
}

pub async fn create_{{ name | lower }}(Json(body): Json<{{ name }}Body>) -> Json<Value> {
    let _ = body;
    Json(serde_json::json!({ "created": true }))
}

pub async fn update_{{ name | lower }}(
    Path(id): Path<String>,
    Json(body): Json<{{ name }}Body>,
) -> Json<Value> {
    let _ = body;
    Json(serde_json::json!({ "id": id, "updated": true }))
}

pub async fn delete_{{ name | lower }}(Path(id): Path<String>) -> Json<Value> {
    Json(serde_json::json!({ "id": id, "deleted": true }))
}
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
