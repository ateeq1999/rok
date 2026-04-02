//! rok-deploy — deployment config generation for the rok ecosystem.
//!
//! Generates Dockerfiles, Kubernetes manifests, and Docker Compose files
//! from a [`DeployConfig`].
//!
//! ```rust
//! use rok_deploy::{DeployConfig, dockerfile, compose};
//!
//! let config = DeployConfig {
//!     name: "my-api".to_string(),
//!     image: "my-api".to_string(),
//!     port: 3000,
//!     ..Default::default()
//! };
//!
//! let df = dockerfile::generate(&config);
//! assert!(df.contains("EXPOSE 3000"));
//!
//! let dc = compose::generate(&config);
//! assert!(dc.contains("my-api"));
//! ```

pub mod compose;
pub mod config;
pub mod dockerfile;
pub mod kubernetes;

pub use config::DeployConfig;
