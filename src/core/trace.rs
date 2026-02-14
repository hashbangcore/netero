use std::path::{Path, PathBuf};
use std::io::Write;
use tokio::net::UnixDatagram;
use terminal_size::terminal_size;

const DEFAULT_TRACE_SOCKET_PATH: &str = "/tmp/netero.trace.sock";

fn resolve_trace_socket_path() -> PathBuf {
    if let Ok(value) = std::env::var("TRACE_SOCKET_PATH") {
        let trimmed = value.trim();
        if !trimmed.is_empty() {
            return PathBuf::from(trimmed);
        }
    }

    if let Ok(value) = std::env::var("XDG_RUNTIME_DIR") {
        let trimmed = value.trim();
        if !trimmed.is_empty() {
            return Path::new(trimmed).join("netero.trace.sock");
        }
    }

    PathBuf::from(DEFAULT_TRACE_SOCKET_PATH)
}

fn separator_line() -> String {
    let width = terminal_size()
        .map(|(w, _)| w.0 as usize)
        .unwrap_or(80);
    let count = width.saturating_sub(1);
    ".".repeat(count) + "\n"
}

pub async fn run_trace_server() -> Result<(), Box<dyn std::error::Error>> {
    let socket_path = resolve_trace_socket_path();

    // Replace old socket if it exists.
    if socket_path.exists() {
        std::fs::remove_file(&socket_path)?;
    }

    let socket = UnixDatagram::bind(&socket_path)?;
    let mut buf = vec![0u8; 64 * 1024];

    let mut counter: u64 = 0;
    let mut stdout = std::io::stdout();

    loop {
        let (len, _) = socket.recv_from(&mut buf).await?;
        let message = String::from_utf8_lossy(&buf[..len]);
        let mut parts = message.splitn(2, '\n');
        let kind = parts.next().unwrap_or("");
        let payload = parts.next().unwrap_or("");

        counter = counter.wrapping_add(1);
        let ts = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
        let header = format!("[id={counter} ts={ts}]\n{kind}\n");
        stdout.write_all(b"\n\n")?;
        let line = separator_line();
        stdout.write_all(line.as_bytes())?;
        stdout.write_all(b"\n\n")?;
        stdout.write_all(header.as_bytes())?;
        stdout.write_all(payload.as_bytes())?;
        if !payload.ends_with('\n') {
            stdout.write_all(b"\n")?;
        }
        stdout.flush()?;
    }
}

pub async fn send_trace(kind: &str, payload: &str) {
    let socket_path = resolve_trace_socket_path();
    let socket = match UnixDatagram::unbound() {
        Ok(sock) => sock,
        Err(_) => return,
    };

    let message = format!("{}\n{}", kind, payload);
    let _ = socket.send_to(message.as_bytes(), socket_path).await;
}
