use futures_util::StreamExt;
use serde_json::json;
use std::io::Write;

use crate::core;

/// Streams a completion response and returns the full collected text.
pub async fn stream_completion(
    service: &core::Service,
    prompt: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let body = json!({
        "model": service.model,
        "messages": [
            { "role": "user", "content": prompt }
        ],
        "stream": true
    });

    let mut req = service.http.post(&service.endpoint).json(&body);

    if let Some(key) = &service.apikey {
        req = req.header("Authorization", format!("Bearer {}", key));
    }

    let response = req.send().await?;
    let mut stream = response.bytes_stream();
    let mut content = String::new();
    let mut stdout = std::io::stdout();

    while let Some(item) = stream.next().await {
        let chunk = item?;
        let text = String::from_utf8_lossy(&chunk);

        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let Some(data) = line.strip_prefix("data:") else {
                continue;
            };
            let data = data.trim();
            if data == "[DONE]" {
                stdout.write_all(b"\n")?;
                stdout.flush()?;
                return Ok(content);
            }
            let parsed: serde_json::Value = serde_json::from_str(data)?;
            let delta = parsed["choices"][0]["delta"]["content"]
                .as_str()
                .unwrap_or("");
            if !delta.is_empty() {
                content.push_str(delta);
                stdout.write_all(delta.as_bytes())?;
                stdout.flush()?;
            }
        }
    }

    stdout.write_all(b"\n")?;
    stdout.flush()?;
    Ok(content)
}
