use serde::{Deserialize, Serialize};

/// Top-level deployment configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployConfig {
    /// Application / service name.
    pub name: String,

    /// Docker image name (without tag).
    pub image: String,

    /// Port the application listens on inside the container.
    pub port: u16,

    /// Docker image tag.  Defaults to `"latest"`.
    pub tag: String,

    /// Rust toolchain target for the release binary.
    pub rust_target: Option<String>,

    /// Number of replicas (Kubernetes).
    pub replicas: u32,

    /// Environment variables to inject at runtime.
    pub env: Vec<EnvVar>,

    /// CPU / memory resource requests (Kubernetes).
    pub resources: Option<Resources>,
}

impl Default for DeployConfig {
    fn default() -> Self {
        Self {
            name: "app".to_string(),
            image: "app".to_string(),
            port: 3000,
            tag: "latest".to_string(),
            rust_target: None,
            replicas: 1,
            env: vec![],
            resources: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvVar {
    pub name: String,
    pub value: String,
}

impl EnvVar {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resources {
    pub cpu_request: String,
    pub memory_request: String,
    pub cpu_limit: String,
    pub memory_limit: String,
}

impl Default for Resources {
    fn default() -> Self {
        Self {
            cpu_request: "100m".to_string(),
            memory_request: "128Mi".to_string(),
            cpu_limit: "500m".to_string(),
            memory_limit: "512Mi".to_string(),
        }
    }
}
