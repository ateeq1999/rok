use crate::schema::StepResult;
use crate::schema::StepTypeResult;
use std::collections::HashMap;
use std::time::Instant;

pub fn run(
    method: &str,
    url: &str,
    headers: &HashMap<String, String>,
    expect_status: u16,
    body: &Option<String>,
    _cwd: &std::path::Path,
) -> StepResult {
    let start = Instant::now();

    let client = reqwest::blocking::Client::new();

    let mut request = match method.to_uppercase().as_str() {
        "GET" => client.get(url),
        "POST" => client.post(url),
        "PUT" => client.put(url),
        "DELETE" => client.delete(url),
        "PATCH" => client.patch(url),
        "HEAD" => client.head(url),
        _ => client.get(url),
    };

    for (key, value) in headers {
        request = request.header(key, value);
    }

    if let Some(body_content) = body {
        request = request.body(body_content.clone());
    }

    let response = request.send();

    let duration_ms = start.elapsed().as_millis() as u64;

    match response {
        Ok(resp) => {
            let status = resp.status().as_u16();
            let body = resp.text().ok();

            let is_ok = status == expect_status;

            StepResult {
                index: 0,
                step_type: StepTypeResult::Http {
                    method: method.to_string(),
                    url: url.to_string(),
                    status,
                    body,
                },
                status: if is_ok {
                    "ok".to_string()
                } else {
                    "error".to_string()
                },
                duration_ms,
                stopped_pipeline: None,
            }
        }
        Err(e) => StepResult {
            index: 0,
            step_type: StepTypeResult::Http {
                method: method.to_string(),
                url: url.to_string(),
                status: 0,
                body: Some(e.to_string()),
            },
            status: "error".to_string(),
            duration_ms,
            stopped_pipeline: None,
        },
    }
}
