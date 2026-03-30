use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateInfo {
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
    pub tags: Vec<String>,
    pub source: TemplateSource,
    pub props: HashMap<String, PropInfo>,
    pub outputs: Vec<TemplateOutput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TemplateSource {
    BuiltIn,
    Project,
    User,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PropInfo {
    pub prop_type: String,
    pub required: bool,
    pub description: String,
    #[serde(default)]
    pub example: Option<String>,
    #[serde(default)]
    pub default: Option<String>,
    #[serde(default)]
    pub values: Option<Vec<String>>,
    #[serde(default)]
    pub pattern: Option<String>,
    #[serde(default)]
    pub min: Option<u32>,
    #[serde(default)]
    pub max: Option<u32>,
    #[serde(default)]
    pub derive_from: Option<String>,
    #[serde(default)]
    pub derive: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateOutput {
    pub from: String,
    pub to: String,
    pub condition: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TemplateSchema {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default = "default_version")]
    pub version: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub output: Vec<TemplateOutput>,
    #[serde(default)]
    pub props: HashMap<String, PropDefinition>,
    #[serde(default)]
    pub hooks: Option<TemplateHooks>,
    #[serde(default)]
    pub post_generate: Vec<PostGenerateAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateHooks {
    #[serde(default)]
    pub before: Option<String>,
    #[serde(default)]
    pub after: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostGenerateAction {
    #[serde(rename = "type")]
    pub action_type: String,
    #[serde(default)]
    pub cmd: Option<String>,
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub tool: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropDefinition {
    #[serde(rename = "type")]
    pub prop_type: String,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub example: Option<String>,
    #[serde(default)]
    pub default: Option<String>,
    #[serde(default)]
    pub values: Option<Vec<String>>,
    #[serde(default)]
    pub pattern: Option<String>,
    #[serde(default)]
    pub min: Option<u32>,
    #[serde(default)]
    pub max: Option<u32>,
    #[serde(default)]
    pub derive_from: Option<String>,
    #[serde(default)]
    pub derive: Option<Vec<String>>,
}

fn default_version() -> String {
    "1.0.0".to_string()
}

fn prop_info(
    prop_type: &str,
    required: bool,
    description: &str,
    example: Option<&str>,
    default: Option<&str>,
    values: Option<Vec<&str>>,
) -> PropInfo {
    PropInfo {
        prop_type: prop_type.to_string(),
        required,
        description: description.to_string(),
        example: example.map(String::from),
        default: default.map(String::from),
        values: values.map(|v| v.into_iter().map(String::from).collect()),
        pattern: None,
        min: None,
        max: None,
        derive_from: None,
        derive: None,
    }
}

pub struct TemplateDiscovery;

impl TemplateDiscovery {
    pub fn discover(cwd: &Path) -> Vec<TemplateInfo> {
        let mut templates = Vec::new();

        templates.extend(Self::discover_builtin());
        templates.extend(Self::discover_project_templates(cwd));
        templates.extend(Self::discover_user_templates());

        templates
    }

    fn discover_builtin() -> Vec<TemplateInfo> {
        vec![
            TemplateInfo {
                name: "react-route".to_string(),
                description: "TanStack React Router file-based route".to_string(),
                version: "1.0.0".to_string(),
                author: "rok".to_string(),
                tags: vec![
                    "react".to_string(),
                    "route".to_string(),
                    "tanstack".to_string(),
                ],
                source: TemplateSource::BuiltIn,
                props: {
                    let mut p = HashMap::new();
                    p.insert(
                        "component".to_string(),
                        prop_info(
                            "string",
                            true,
                            "Component name",
                            Some("Dashboard"),
                            None,
                            None,
                        ),
                    );
                    p.insert(
                        "path".to_string(),
                        prop_info("string", true, "Route path", Some("/dashboard"), None, None),
                    );
                    p
                },
                outputs: vec![TemplateOutput {
                    from: "react-route".to_string(),
                    to: "{{dir}}/{{kebab_case name}}.tsx".to_string(),
                    condition: None,
                }],
            },
            TemplateInfo {
                name: "react-component".to_string(),
                description: "React functional component with TypeScript".to_string(),
                version: "1.0.0".to_string(),
                author: "rok".to_string(),
                tags: vec![
                    "react".to_string(),
                    "component".to_string(),
                    "typescript".to_string(),
                ],
                source: TemplateSource::BuiltIn,
                props: {
                    let mut p = HashMap::new();
                    p.insert(
                        "name".to_string(),
                        prop_info(
                            "string",
                            true,
                            "Component name (PascalCase)",
                            Some("Button"),
                            None,
                            None,
                        ),
                    );
                    p
                },
                outputs: vec![TemplateOutput {
                    from: "react-component".to_string(),
                    to: "{{kebab_case name}}.tsx".to_string(),
                    condition: None,
                }],
            },
            TemplateInfo {
                name: "api-handler".to_string(),
                description: "TanStack Start API handler".to_string(),
                version: "1.0.0".to_string(),
                author: "rok".to_string(),
                tags: vec![
                    "api".to_string(),
                    "handler".to_string(),
                    "tanstack".to_string(),
                ],
                source: TemplateSource::BuiltIn,
                props: {
                    let mut p = HashMap::new();
                    p.insert(
                        "name".to_string(),
                        prop_info("string", true, "Handler name", Some("getUsers"), None, None),
                    );
                    p
                },
                outputs: vec![TemplateOutput {
                    from: "api-handler".to_string(),
                    to: "{{kebab_case name}}.ts".to_string(),
                    condition: None,
                }],
            },
            TemplateInfo {
                name: "rust-module".to_string(),
                description: "Rust module with greeting function".to_string(),
                version: "1.0.0".to_string(),
                author: "rok".to_string(),
                tags: vec!["rust".to_string(), "module".to_string()],
                source: TemplateSource::BuiltIn,
                props: {
                    let mut p = HashMap::new();
                    p.insert(
                        "name".to_string(),
                        prop_info("string", true, "Module name", Some("greeting"), None, None),
                    );
                    p
                },
                outputs: vec![TemplateOutput {
                    from: "rust-module".to_string(),
                    to: "{{snake_case name}}.rs".to_string(),
                    condition: None,
                }],
            },
            TemplateInfo {
                name: "test-file".to_string(),
                description: "Vitest test file".to_string(),
                version: "1.0.0".to_string(),
                author: "rok".to_string(),
                tags: vec!["test".to_string(), "vitest".to_string()],
                source: TemplateSource::BuiltIn,
                props: {
                    let mut p = HashMap::new();
                    p.insert(
                        "name".to_string(),
                        prop_info(
                            "string",
                            true,
                            "Test subject name",
                            Some("MyComponent"),
                            None,
                            None,
                        ),
                    );
                    p
                },
                outputs: vec![TemplateOutput {
                    from: "test-file".to_string(),
                    to: "{{kebab_case name}}.test.ts".to_string(),
                    condition: None,
                }],
            },
        ]
    }

    fn discover_project_templates(cwd: &Path) -> Vec<TemplateInfo> {
        let mut templates = Vec::new();
        let project_templates_dir = cwd.join(".rok").join("templates");

        if project_templates_dir.exists() {
            templates.extend(Self::scan_templates_dir(
                &project_templates_dir,
                TemplateSource::Project,
            ));
        }

        templates
    }

    fn discover_user_templates() -> Vec<TemplateInfo> {
        let mut templates = Vec::new();

        if let Some(home) = dirs::home_dir() {
            let user_templates_dir = home.join(".rok").join("templates");
            if user_templates_dir.exists() {
                templates.extend(Self::scan_templates_dir(
                    &user_templates_dir,
                    TemplateSource::User,
                ));
            }
        }

        templates
    }

    fn scan_templates_dir(dir: &Path, source: TemplateSource) -> Vec<TemplateInfo> {
        let mut templates = Vec::new();

        for entry in WalkDir::new(dir)
            .max_depth(2)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file()
                && path
                    .file_name()
                    .map(|n| n == ".rok-template.json")
                    .unwrap_or(false)
            {
                if let Ok(schema) = Self::load_template_schema(path) {
                    templates.push(TemplateInfo {
                        name: schema.name,
                        description: schema.description,
                        version: schema.version,
                        author: schema.author,
                        tags: schema.tags,
                        source: source.clone(),
                        props: schema
                            .props
                            .into_iter()
                            .map(|(k, v)| {
                                (
                                    k,
                                    PropInfo {
                                        prop_type: v.prop_type,
                                        required: v.required,
                                        description: v.description,
                                        example: v.example,
                                        default: v.default,
                                        values: v.values,
                                        pattern: v.pattern,
                                        min: v.min,
                                        max: v.max,
                                        derive_from: v.derive_from,
                                        derive: v.derive,
                                    },
                                )
                            })
                            .collect(),
                        outputs: schema.output,
                    });
                }
            }
        }

        templates
    }

    fn load_template_schema(path: &Path) -> Result<TemplateSchema, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let schema: TemplateSchema = serde_json::from_str(&content)?;
        Ok(schema)
    }
}

pub fn list_templates(cwd: &Path) -> Vec<TemplateInfo> {
    TemplateDiscovery::discover(cwd)
}
