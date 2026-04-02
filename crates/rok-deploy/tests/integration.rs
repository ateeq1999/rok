use rok_deploy::{
    compose,
    config::{DeployConfig, EnvVar, Resources},
    dockerfile, kubernetes,
};

fn cfg() -> DeployConfig {
    DeployConfig {
        name: "api".to_string(),
        image: "ghcr.io/org/api".to_string(),
        port: 3000,
        tag: "v1.2.3".to_string(),
        replicas: 2,
        env: vec![
            EnvVar::new("DATABASE_URL", "postgres://localhost/api"),
            EnvVar::new("RUST_LOG", "info"),
        ],
        resources: Some(Resources::default()),
        ..Default::default()
    }
}

#[test]
fn dockerfile_structure() {
    let df = dockerfile::generate(&cfg());
    assert!(df.contains("FROM rust:1-slim AS builder"));
    assert!(df.contains("FROM debian:bookworm-slim AS runtime"));
    assert!(df.contains("EXPOSE 3000"));
    assert!(df.contains("ENV DATABASE_URL=postgres://localhost/api"));
    assert!(df.contains("ENV RUST_LOG=info"));
}

#[test]
fn compose_structure() {
    let dc = compose::generate(&cfg());
    assert!(dc.contains(r#"image: ghcr.io/org/api:v1.2.3"#));
    assert!(dc.contains("3000:3000"));
    assert!(dc.contains("RUST_LOG=info"));
}

#[test]
fn k8s_deployment() {
    let d = kubernetes::deployment(&cfg());
    assert!(d.contains("kind: Deployment"));
    assert!(d.contains("replicas: 2"));
    assert!(d.contains("image: ghcr.io/org/api:v1.2.3"));
    assert!(d.contains("containerPort: 3000"));
    assert!(d.contains("name: DATABASE_URL"));
}

#[test]
fn k8s_service() {
    let s = kubernetes::service(&cfg());
    assert!(s.contains("kind: Service"));
    assert!(s.contains("port: 3000"));
    assert!(s.contains("targetPort: 3000"));
}

#[test]
fn k8s_manifests_combined() {
    let m = kubernetes::manifests(&cfg());
    assert!(m.contains("kind: Deployment"));
    assert!(m.contains("kind: Service"));
    assert!(m.contains("---"));
}

#[test]
fn default_config_is_sane() {
    let cfg = DeployConfig::default();
    assert_eq!(cfg.port, 3000);
    assert_eq!(cfg.replicas, 1);
    assert_eq!(cfg.tag, "latest");
}
