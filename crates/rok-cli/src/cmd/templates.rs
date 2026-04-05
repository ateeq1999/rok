use crate::steps;

pub fn list() -> i32 {
    let templates = steps::template_discovery::list_templates(std::path::Path::new("."));
    let json = serde_json::to_string_pretty(&templates).unwrap_or_default();
    println!("{}", json);
    0
}

pub fn init(name: Option<String>) -> i32 {
    let template_name = name.unwrap_or_else(|| {
        println!("Enter template name: ");
        super::read_input()
    });

    println!("Creating template: {}", template_name);

    let template_dir = std::path::Path::new(".rok/templates").join(&template_name);
    if let Err(e) = std::fs::create_dir_all(&template_dir) {
        eprintln!("Failed to create template directory: {}", e);
        return 1;
    }

    let schema = steps::template_discovery::TemplateSchema {
        name: template_name.clone(),
        description: String::new(),
        version: "1.0.0".to_string(),
        author: String::new(),
        tags: vec![],
        extends: None,
        output: vec![steps::template_discovery::TemplateOutput {
            from: "template.txt".to_string(),
            to: "{{name}}.txt".to_string(),
            condition: None,
        }],
        props: std::collections::HashMap::new(),
        hooks: None,
        post_generate: vec![],
    };

    let schema_path = template_dir.join(".rok-template.json");
    let schema_json = serde_json::to_string_pretty(&schema).expect("Failed to serialize schema");
    std::fs::write(&schema_path, &schema_json).expect("Failed to write schema");
    std::fs::write(template_dir.join("template.txt"), "{{ name }}\n")
        .expect("Failed to write template");

    println!("Template created at: {}", template_dir.display());
    println!("   Edit .rok-template.json to add props");
    0
}

pub fn validate(path: Option<String>) -> i32 {
    let template_path = path.unwrap_or_else(|| ".".to_string());
    let cwd = std::path::Path::new(&template_path);

    let template_file = if cwd.is_file() {
        cwd.to_path_buf()
    } else {
        cwd.join(".rok-template.json")
    };

    if !template_file.exists() {
        eprintln!("Template schema not found: {}", template_file.display());
        return 1;
    }

    match steps::template_discovery::validate_template(&template_file) {
        Ok(_) => {
            println!("Template is valid!");
            0
        }
        Err(e) => {
            eprintln!("Template validation failed:");
            eprintln!("{}", e);
            1
        }
    }
}
