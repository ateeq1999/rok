//! Docker Compose file generation.

use crate::config::DeployConfig;

/// Generate a `docker-compose.yml` for local development.
pub fn generate(config: &DeployConfig) -> String {
    let env_lines: String = config
        .env
        .iter()
        .map(|e| format!("      - {}={}\n", e.name, e.value))
        .collect();

    let env_section = if env_lines.is_empty() {
        String::new()
    } else {
        format!("    environment:\n{env_lines}")
    };

    format!(
        r#"version: "3.9"

services:
  {name}:
    build: .
    image: {image}:{tag}
    ports:
      - "{port}:{port}"
{env_section}    restart: unless-stopped
"#,
        name = config.name,
        image = config.image,
        tag = config.tag,
        port = config.port,
        env_section = env_section,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_service_name() {
        let cfg = DeployConfig {
            name: "my-api".to_string(),
            ..Default::default()
        };
        let dc = generate(&cfg);
        assert!(dc.contains("my-api"));
    }

    #[test]
    fn contains_port_mapping() {
        let cfg = DeployConfig {
            port: 4000,
            ..Default::default()
        };
        let dc = generate(&cfg);
        assert!(dc.contains("4000:4000"));
    }
}
