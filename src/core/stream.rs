//! Streaming chat completions: turning SSE payloads into text.

use std::io::Write;

use crate::core::router::{Service, ServiceError, parse_response, trace_failed_response};
use crate::core::trace::send_trace;
use futures_util::StreamExt;

/// What a single `data:` payload from the stream means.
enum Line {
    /// Nothing to write: a keep-alive or an empty delta.
    Skip,
    /// The stream is complete.
    Done,
    /// Text to append to the answer.
    Text(String),
    /// A payload that could not be parsed, kept for the final error message.
    Malformed(String),
}

/// Interprets one `data:` payload from the stream.
fn interpret(data: &str) -> Line {
    if data == "[DONE]" {
        return Line::Done;
    }

    match serde_json::from_str::<serde_json::Value>(data) {
        Ok(parsed) => {
            match parsed
                .pointer("/choices/0/delta/content")
                .and_then(|value| value.as_str())
            {
                Some(delta) if !delta.is_empty() => Line::Text(delta.to_string()),
                _ => Line::Skip,
            }
        }
        Err(_) => Line::Malformed(data.to_string()),
    }
}

/// Splits complete lines out of the buffer, keeping any partial line for the next chunk.
///
/// A chunk can split a line in half, so only the text up to the last newline is consumed.
fn take_lines(buffer: &mut String) -> Vec<String> {
    let mut lines = Vec::new();
    while let Some(end) = buffer.find('\n') {
        lines.push(buffer.drain(..=end).collect::<String>());
    }
    lines
}

impl Service {
    /// Streams an answer into `out` and returns the full collected text.
    pub async fn complete_stream(
        &self,
        content: &str,
        out: &mut impl Write,
    ) -> Result<String, ServiceError> {
        send_trace(":: REQUEST ::", content).await;

        let response = self
            .request(content, Some(true))
            .send()
            .await
            .map_err(|source| ServiceError::Transport {
                endpoint: self.endpoint().to_string(),
                source,
            })?;

        let status = response.status();
        if !status.is_success() {
            let body = response
                .text()
                .await
                .map_err(|source| ServiceError::Read { source })?;
            trace_failed_response(status, &body).await;
            return parse_response(status, &body);
        }

        let mut stream = response.bytes_stream();
        let mut content = String::new();
        let mut buffer = String::new();
        let mut malformed: Option<String> = None;

        while let Some(item) = stream.next().await {
            let chunk = item.map_err(|source| ServiceError::Read { source })?;
            buffer.push_str(&String::from_utf8_lossy(&chunk));

            for line in take_lines(&mut buffer) {
                let line = line.trim();
                let Some(data) = line.strip_prefix("data:") else {
                    continue;
                };
                match interpret(data.trim()) {
                    Line::Done => return finish(out, content).await,
                    Line::Text(delta) => {
                        content.push_str(&delta);
                        write(out, &delta)?;
                    }
                    Line::Malformed(payload) => malformed = Some(payload),
                    Line::Skip => {}
                }
            }
        }

        if content.is_empty() {
            return Err(match malformed {
                Some(payload) => ServiceError::UnexpectedBody {
                    reason: "the stream carried no deltas".to_string(),
                    snippet: payload,
                },
                None => ServiceError::EmptyContent,
            });
        }

        finish(out, content).await
    }
}

/// Writes one delta and flushes it, so the answer appears as it arrives.
fn write(out: &mut impl Write, delta: &str) -> Result<(), ServiceError> {
    out.write_all(delta.as_bytes())
        .and_then(|()| out.flush())
        .map_err(|source| ServiceError::Write { source })
}

/// Closes a streamed answer with a newline.
async fn finish(out: &mut impl Write, content: String) -> Result<String, ServiceError> {
    out.write_all(b"\n")
        .map_err(|source| ServiceError::Write { source })?;
    send_trace(":: RESPONSE ::", &content).await;
    Ok(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn takes_complete_lines_and_keeps_the_partial_one() {
        let mut buffer = String::from("data: {\"a\":1}\ndata: {\"b");
        assert_eq!(take_lines(&mut buffer), vec!["data: {\"a\":1}\n"]);
        assert_eq!(buffer, "data: {\"b");
    }

    #[test]
    fn reassembles_a_payload_split_across_chunks() {
        let mut buffer = String::new();
        let mut deltas = Vec::new();

        for chunk in [
            "data: {\"choices\":[{\"delta\":{\"cont",
            "ent\":\"Hola\"}}]}\n",
        ] {
            buffer.push_str(chunk);
            for line in take_lines(&mut buffer) {
                let Some(data) = line.trim().strip_prefix("data:") else {
                    continue;
                };
                if let Line::Text(delta) = interpret(data.trim()) {
                    deltas.push(delta);
                }
            }
        }

        assert_eq!(deltas.concat(), "Hola");
        assert!(buffer.is_empty());
    }

    #[test]
    fn reads_deltas_and_recognizes_the_end() {
        let delta = r#"{"choices":[{"delta":{"content":"Hola"}}]}"#;
        assert!(matches!(interpret(delta), Line::Text(text) if text == "Hola"));
        assert!(matches!(interpret("[DONE]"), Line::Done));
    }

    #[test]
    fn ignores_payloads_without_text() {
        assert!(matches!(
            interpret(r#"{"choices":[{"delta":{}}]}"#),
            Line::Skip
        ));
        assert!(matches!(interpret(r#"{"choices":[]}"#), Line::Skip));
        assert!(matches!(interpret(r#"{"error":"boom"}"#), Line::Skip));
    }

    #[test]
    fn keeps_malformed_payloads_for_the_error_message() {
        let line = interpret("not json at all");
        assert!(matches!(line, Line::Malformed(payload) if payload == "not json at all"));
    }

    #[test]
    fn writes_the_answer_to_the_sink() {
        let mut out = Vec::new();
        write(&mut out, "Hola").unwrap();
        assert_eq!(out, b"Hola");
    }
}
