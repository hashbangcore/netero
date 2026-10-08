use std::process::{Command, ExitStatus, Stdio};

/// Interpreter used to run a command string.
#[derive(Clone, Copy)]
pub enum Shell {
    /// POSIX shell without login semantics (`sh -c`).
    Posix,
    /// Bash as a login shell (`bash -lc`).
    BashLogin,
}

/// Captured result of a shell command.
pub struct CommandOutput {
    /// True when the command ran and exited successfully.
    pub success: bool,
    /// Exit status rendered as text.
    pub status: String,
    /// Raw standard output.
    pub stdout: String,
    /// Raw standard error.
    pub stderr: String,
    /// Error text when the interpreter could not be started.
    pub error: Option<String>,
}

impl CommandOutput {
    /// Returns standard output, or a placeholder when the command produced none.
    pub fn stdout_display(&self) -> &str {
        if self.stdout.trim_end().is_empty() {
            "<empty>"
        } else {
            self.stdout.trim_end()
        }
    }

    /// Returns standard error, or a placeholder when the command produced none.
    pub fn stderr_display(&self) -> &str {
        if self.stderr.trim_end().is_empty() {
            "<empty>"
        } else {
            self.stderr.trim_end()
        }
    }
}

/// Runs a command string through the given shell and captures its output.
pub fn run_shell(command: &str, shell: Shell) -> CommandOutput {
    let mut cmd = match shell {
        Shell::Posix => {
            let mut cmd = Command::new("sh");
            cmd.arg("-c");
            cmd
        }
        Shell::BashLogin => {
            let mut cmd = Command::new("bash");
            cmd.args(["-lc"]);
            cmd
        }
    };

    let result = cmd
        .arg(command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output();

    match result {
        Ok(out) => from_output(out.status, &out.stdout, &out.stderr),
        Err(err) => CommandOutput {
            success: false,
            status: String::new(),
            stdout: String::new(),
            stderr: String::new(),
            error: Some(err.to_string()),
        },
    }
}

/// Builds a `CommandOutput` from raw process output.
fn from_output(status: ExitStatus, stdout: &[u8], stderr: &[u8]) -> CommandOutput {
    CommandOutput {
        success: status.success(),
        status: status.to_string(),
        stdout: String::from_utf8_lossy(stdout).to_string(),
        stderr: String::from_utf8_lossy(stderr).to_string(),
        error: None,
    }
}
