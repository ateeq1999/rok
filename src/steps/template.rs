use crate::schema::StepResult;
use crate::schema::StepTypeResult;
use crate::steps::template_discovery;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::Instant;
use tera::{Tera, Value};

pub fn run(
    name: &str,
    builtin: &str,
    source: &str,
    output: &str,
    vars: &HashMap<String, String>,
    cwd: &Path,
) -> StepResult {
    let start = Instant::now();

    let template_content = if !name.is_empty() {
        get_custom_template(name, cwd)
    } else if !builtin.is_empty() {
        get_builtin_template(builtin)
    } else if !source.is_empty() {
        fs::read_to_string(cwd.join(source)).unwrap_or_default()
    } else {
        String::new()
    };

    if template_content.is_empty() {
        return StepResult {
            index: 0,
            step_type: StepTypeResult::Template {
                output: output.to_string(),
                rendered: false,
            },
            status: "error".to_string(),
            duration_ms: 0,
            stopped_pipeline: None,
        };
    }

    let mut tera = Tera::default();
    register_filters(&mut tera);

    let rendered = tera
        .render_str(
            &template_content,
            &tera::Context::from_serialize(vars).unwrap_or_default(),
        )
        .unwrap_or(template_content);

    let output_path = cwd.join(output);

    let result = if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)
    } else {
        Ok(())
    }
    .and_then(|_| fs::write(&output_path, &rendered));

    let duration_ms = start.elapsed().as_millis() as u64;

    match result {
        Ok(_) => StepResult {
            index: 0,
            step_type: StepTypeResult::Template {
                output: output.to_string(),
                rendered: true,
            },
            status: "ok".to_string(),
            duration_ms,
            stopped_pipeline: None,
        },
        Err(_e) => StepResult {
            index: 0,
            step_type: StepTypeResult::Template {
                output: output.to_string(),
                rendered: false,
            },
            status: "error".to_string(),
            duration_ms,
            stopped_pipeline: None,
        },
    }
}

fn get_custom_template(name: &str, cwd: &Path) -> String {
    let templates = template_discovery::list_templates(cwd);

    for template in templates {
        if template.name == name {
            if let Some(first_output) = template.outputs.first() {
                let template_dir = cwd.join(".rok").join("templates").join(&template.name);
                let template_file = template_dir.join(&first_output.from);
                if template_file.exists() {
                    return fs::read_to_string(&template_file).unwrap_or_default();
                }
            }
        }
    }

    String::new()
}

fn get_builtin_template(name: &str) -> String {
    match name {
        "react-route" => r#"import { createFileRoute } from '@tanstack/react-router';
import Component from '{{component}}';

export const Route = createFileRoute('{{path}}')({
  component: Component,
});
"#
        .to_string(),
        "react-component" => r#"import React from 'react';

interface {{name}}Props {
  className?: string;
}

export function {{name}}({ className }: {{name}}Props) {
  return (
    <div className={className}>
      {{name}} Component
    </div>
  );
}
"#
        .to_string(),
        "api-handler" => r#"import { json } from '@tanstack/start';

export async function handler(request: Request) {
  const data = {
    message: 'Hello from {{name}}',
    timestamp: new Date().toISOString(),
  };
  
  return json(data);
}
"#
        .to_string(),
        "rust-module" => r#"pub mod {{name}} {
    pub fn greet() -> &'static str {
        "Hello from {{name}}!"
    }
}
"#
        .to_string(),
        "test-file" => r#"import { describe, it, expect } from 'vitest';

describe('{{name}}', () => {
  it('should work', () => {
    expect(true).toBe(true);
  });
});
"#
        .to_string(),
        _ => String::new(),
    }
}

fn register_filters(tera: &mut Tera) {
    tera.register_filter("camelcase", |value: &Value, _: &HashMap<String, Value>| {
        let s = value.as_str().unwrap_or("");
        let words: Vec<&str> = s
            .split(|c: char| !c.is_alphanumeric())
            .filter(|w| !w.is_empty())
            .collect();
        let mut result = String::new();
        for (i, word) in words.iter().enumerate() {
            if i == 0 {
                result.push_str(&word.to_lowercase());
            } else {
                let mut chars = word.chars();
                if let Some(first) = chars.next() {
                    result.push(first.to_uppercase().next().unwrap_or(first));
                    result.extend(chars.flat_map(|c| c.to_lowercase()));
                }
            }
        }
        Ok(Value::String(result))
    });

    tera.register_filter("snakecase", |value: &Value, _: &HashMap<String, Value>| {
        let s = value.as_str().unwrap_or("");
        let mut result = String::new();
        for (i, c) in s.chars().enumerate() {
            if c.is_uppercase() && i > 0 {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap_or(c));
        }
        result = result.split_whitespace().collect::<Vec<_>>().join("_");
        Ok(Value::String(result))
    });

    tera.register_filter("kebabcase", |value: &Value, _: &HashMap<String, Value>| {
        let s = value.as_str().unwrap_or("");
        let mut result = String::new();
        for (i, c) in s.chars().enumerate() {
            if c.is_uppercase() && i > 0 {
                result.push('-');
            }
            result.push(c.to_lowercase().next().unwrap_or(c));
        }
        result = result.split_whitespace().collect::<Vec<_>>().join("-");
        Ok(Value::String(result))
    });

    tera.register_filter("pascalcase", |value: &Value, _: &HashMap<String, Value>| {
        let s = value.as_str().unwrap_or("");
        let words: Vec<&str> = s
            .split(|c: char| !c.is_alphanumeric())
            .filter(|w| !w.is_empty())
            .collect();
        let mut result = String::new();
        for word in words {
            let mut chars = word.chars();
            if let Some(first) = chars.next() {
                result.push(first.to_uppercase().next().unwrap_or(first));
                result.extend(chars.flat_map(|c| c.to_lowercase()));
            }
        }
        Ok(Value::String(result))
    });

    tera.register_filter("pluralize", |value: &Value, _: &HashMap<String, Value>| {
        let s = value.as_str().unwrap_or("");
        let plural = if s.ends_with("s")
            || s.ends_with("x")
            || s.ends_with("z")
            || s.ends_with("ch")
            || s.ends_with("sh")
        {
            format!("{}es", s)
        } else if s.ends_with("y")
            && s.len() > 1
            && !is_vowel(s.chars().nth(s.len() - 2).unwrap_or('a'))
        {
            format!("{}ies", &s[..s.len() - 1])
        } else {
            format!("{}s", s)
        };
        Ok(Value::String(plural))
    });

    tera.register_filter(
        "singularize",
        |value: &Value, _: &HashMap<String, Value>| {
            let s = value.as_str().unwrap_or("");
            let singular = if let Some(stripped) = s.strip_suffix("ies") {
                format!("{}y", stripped)
            } else if let Some(stripped) = s.strip_suffix("es") {
                if !s.ends_with("ss") {
                    stripped.to_string()
                } else {
                    s.to_string()
                }
            } else if let Some(stripped) = s.strip_suffix("s") {
                if !s.ends_with("ss") {
                    stripped.to_string()
                } else {
                    s.to_string()
                }
            } else {
                s.to_string()
            };
            Ok(Value::String(singular))
        },
    );

    tera.register_filter("uppercase", |value: &Value, _: &HashMap<String, Value>| {
        Ok(Value::String(value.as_str().unwrap_or("").to_uppercase()))
    });

    tera.register_filter("lowercase", |value: &Value, _: &HashMap<String, Value>| {
        Ok(Value::String(value.as_str().unwrap_or("").to_lowercase()))
    });

    tera.register_filter("capitalize", |value: &Value, _: &HashMap<String, Value>| {
        let s = value.as_str().unwrap_or("");
        let mut chars = s.chars();
        match chars.next() {
            None => Ok(Value::String(String::new())),
            Some(first) => {
                let rest: String = chars.flat_map(|c| c.to_lowercase()).collect();
                Ok(Value::String(format!(
                    "{}{}",
                    first.to_uppercase().next().unwrap_or(first),
                    rest
                )))
            }
        }
    });
}

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U')
}
