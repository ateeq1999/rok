//! Dockerfile generation.

use crate::config::DeployConfig;

/// Generate a multi-stage Dockerfile for a Rust application.
pub fn generate(config: &DeployConfig) -> String {
    let target_flag = config
        .rust_target
        .as_deref()
        .map(|t| format!(" --target {t}"))
        .unwrap_or_default();

    let env_lines: String = config
        .env
        .iter()
        .map(|e| format!("ENV {}={}\n", e.name, e.value))
        .collect();

    format!(
        r#"# ── Build stage ──────────────────────────────────────────────────────────────
FROM rust:1-slim AS builder
WORKDIR /app

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {{}}" > src/main.rs
RUN cargo build --release{target_flag} && rm -rf src

# Build application
COPY . .
RUN touch src/main.rs && cargo build --release{target_flag}

# ── Runtime stage ─────────────────────────────────────────────────────────────
FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app

COPY --from=builder /app/target/release/{name} /app/{name}

{env_lines}EXPOSE {port}

CMD ["/app/{name}"]
"#,
        name = config.name,
        port = config.port,
        target_flag = target_flag,
        env_lines = env_lines,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{DeployConfig, EnvVar};

    #[test]
    fn contains_expose() {
        let cfg = DeployConfig { port: 8080, ..Default::default() };
        let df = generate(&cfg);
        assert!(df.contains("EXPOSE 8080"));
    }

    #[test]
    fn contains_env_vars() {
        let cfg = DeployConfig {
            env: vec![EnvVar::new("RUST_LOG", "info")],
            ..Default::default()
        };
        let df = generate(&cfg);
        assert!(df.contains("ENV RUST_LOG=info"));
    }

    #[test]
    fn multi_stage_structure() {
        let cfg = DeployConfig::default();
        let df = generate(&cfg);
        assert!(df.contains("AS builder"));
        assert!(df.contains("AS runtime"));
    }
}
