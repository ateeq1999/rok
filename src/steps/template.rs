use crate::schema::StepResult;
use crate::schema::StepTypeResult;
use crate::steps::template_discovery;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::Instant;
use tera::Tera;

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
