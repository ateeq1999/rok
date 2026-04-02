//! Kubernetes manifest generation (Deployment + Service).

use crate::config::DeployConfig;

/// Generate a Kubernetes `Deployment` manifest.
pub fn deployment(config: &DeployConfig) -> String {
    let env_section = if config.env.is_empty() {
        String::new()
    } else {
        let vars: String = config
            .env
            .iter()
            .map(|e| {
                format!(
                    "        - name: {}\n          value: \"{}\"\n",
                    e.name, e.value
                )
            })
            .collect();
        format!("        env:\n{vars}")
    };

    let resources_section = config
        .resources
        .as_ref()
        .map(|r| {
            format!(
                r#"        resources:
          requests:
            cpu: {}
            memory: {}
          limits:
            cpu: {}
            memory: {}
"#,
                r.cpu_request, r.memory_request, r.cpu_limit, r.memory_limit
            )
        })
        .unwrap_or_default();

    format!(
        r#"apiVersion: apps/v1
kind: Deployment
metadata:
  name: {name}
  labels:
    app: {name}
spec:
  replicas: {replicas}
  selector:
    matchLabels:
      app: {name}
  template:
    metadata:
      labels:
        app: {name}
    spec:
      containers:
        - name: {name}
          image: {image}:{tag}
          ports:
            - containerPort: {port}
{env_section}{resources}
"#,
        name = config.name,
        image = config.image,
        tag = config.tag,
        port = config.port,
        replicas = config.replicas,
        env_section = env_section,
        resources = resources_section,
    )
}

/// Generate a Kubernetes `Service` manifest (ClusterIP).
pub fn service(config: &DeployConfig) -> String {
    format!(
        r#"apiVersion: v1
kind: Service
metadata:
  name: {name}
spec:
  selector:
    app: {name}
  ports:
    - protocol: TCP
      port: {port}
      targetPort: {port}
  type: ClusterIP
"#,
        name = config.name,
        port = config.port,
    )
}

/// Generate both Deployment and Service separated by `---`.
pub fn manifests(config: &DeployConfig) -> String {
    format!("{}\n---\n{}", deployment(config), service(config))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::DeployConfig;

    #[test]
    fn deployment_contains_image() {
        let cfg = DeployConfig {
            image: "my-app".to_string(),
            tag: "v1.0".to_string(),
            ..Default::default()
        };
        let d = deployment(&cfg);
        assert!(d.contains("my-app:v1.0"));
    }

    #[test]
    fn service_contains_port() {
        let cfg = DeployConfig {
            port: 9000,
            ..Default::default()
        };
        let s = service(&cfg);
        assert!(s.contains("9000"));
    }

    #[test]
    fn manifests_has_separator() {
        let cfg = DeployConfig::default();
        let m = manifests(&cfg);
        assert!(m.contains("---"));
        assert!(m.contains("kind: Deployment"));
        assert!(m.contains("kind: Service"));
    }
}
