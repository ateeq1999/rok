//! rok-generate — Tera-based code generation for the rok ecosystem.
//!
//! ```rust
//! use rok_generate::Generator;
//! use std::collections::HashMap;
//!
//! let gen = Generator::new();
//! let mut vars = HashMap::new();
//! vars.insert("name".to_string(), serde_json::json!("User"));
//! vars.insert("fields".to_string(), serde_json::json!(["id", "name", "email"]));
//!
//! let code = gen.render("model", &vars).unwrap();
//! assert!(code.contains("struct User"));
//! ```

pub mod error;
pub mod generator;
pub mod templates;

pub use error::GenerateError;
pub use generator::Generator;
