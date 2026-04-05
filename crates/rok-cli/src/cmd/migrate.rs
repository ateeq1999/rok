use crate::cli::MigrateCommands;
use rok_migrate::{load_from_dir, Migrator};

pub fn run(command: &MigrateCommands) -> i32 {
    match command {
        MigrateCommands::Init => {
            println!("{}", Migrator::history_table_sql());
            0
        }
        MigrateCommands::Status { dir } => status(dir),
        MigrateCommands::Plan { dir } => plan(dir),
        MigrateCommands::New { name, dir } => new(name, dir),
    }
}

fn status(dir: &std::path::Path) -> i32 {
    let migrations = match load_from_dir(dir) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Error loading migrations: {}", e);
            return 1;
        }
    };

    if migrations.is_empty() {
        println!("No migrations found in {}", dir.display());
    } else {
        println!("Migrations in {}:", dir.display());
        for m in &migrations {
            let reversible = if m.down_sql.is_some() { "reversible" } else { "irreversible" };
            println!("  {:04}  {}  [{}]", m.version, m.name, reversible);
        }
    }
    0
}

fn plan(dir: &std::path::Path) -> i32 {
    let migrations = match load_from_dir(dir) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Error loading migrations: {}", e);
            return 1;
        }
    };

    let mut migrator = Migrator::new();
    for m in migrations {
        if let Err(e) = migrator.add(m) {
            eprintln!("Error: {}", e);
            return 1;
        }
    }

    let steps = migrator.plan_up(&[]);
    if steps.is_empty() {
        println!("No pending migrations.");
    } else {
        println!("Execution plan ({} migration(s)):\n", steps.len());
        for step in &steps {
            println!("-- {:04}: {}", step.migration.version, step.migration.name);
            println!("{}\n", step.sql);
        }
    }
    0
}

fn new(name: &str, dir: &std::path::Path) -> i32 {
    let next_version = load_from_dir(dir)
        .unwrap_or_default()
        .iter()
        .map(|m| m.version)
        .max()
        .unwrap_or(0)
        + 1;

    if let Err(e) = std::fs::create_dir_all(dir) {
        eprintln!("Error creating migrations directory: {}", e);
        return 1;
    }

    let snake_name = rok_utils::string::to_snake_case(name);
    let path = dir.join(format!("{:04}_{}.sql", next_version, snake_name));

    if let Err(e) = std::fs::write(&path, "-- UP\n\n\n-- DOWN\n\n") {
        eprintln!("Error writing migration file: {}", e);
        return 1;
    }

    println!("created  {}", path.display());
    0
}
