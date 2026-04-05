use crate::cli::DeployCommands;
use rok_deploy::{compose, dockerfile, kubernetes, DeployConfig};

pub fn run(command: &DeployCommands) -> i32 {
    match command {
        DeployCommands::Dockerfile { name, port } => {
            println!("{}", dockerfile::generate(&cfg(name, *port, 1)));
        }
        DeployCommands::Compose { name, port } => {
            println!("{}", compose::generate(&cfg(name, *port, 1)));
        }
        DeployCommands::K8s { name, port, replicas } => {
            println!("{}", kubernetes::manifests(&cfg(name, *port, *replicas)));
        }
        DeployCommands::All { name, port, replicas } => {
            let c = cfg(name, *port, *replicas);
            println!("# ── Dockerfile ──────────────────────────────────────────────────────────────");
            println!("{}", dockerfile::generate(&c));
            println!("# ── docker-compose.yml ──────────────────────────────────────────────────────");
            println!("{}", compose::generate(&c));
            println!("# ── kubernetes manifests ────────────────────────────────────────────────────");
            println!("{}", kubernetes::manifests(&c));
        }
    }
    0
}

fn cfg(name: &str, port: u16, replicas: u32) -> DeployConfig {
    DeployConfig {
        name: name.to_string(),
        image: name.to_string(),
        port,
        replicas,
        ..Default::default()
    }
}
