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

#[cfg(test)]
mod tests {
    use super::*;

    fn output(stdout: &str, stderr: &str) -> CommandOutput {
        CommandOutput {
            success: true,
            status: "exit status: 0".to_string(),
            stdout: stdout.to_string(),
            stderr: stderr.to_string(),
            error: None,
        }
    }

    #[test]
    fn empty_output_becomes_a_placeholder() {
        let out = output("", "   \n");
        assert_eq!(out.stdout_display(), "<empty>");
        assert_eq!(out.stderr_display(), "<empty>");
    }

    #[test]
    fn display_trims_trailing_whitespace() {
        let out = output("hola\n\n", "  aviso  \n");
        assert_eq!(out.stdout_display(), "hola");
        assert_eq!(out.stderr_display(), "  aviso");
    }

    #[test]
    fn captures_stdout_of_a_successful_command() {
        let out = run_shell("echo hola", Shell::Posix);
        assert!(out.success);
        assert!(out.error.is_none());
        assert_eq!(out.stdout, "hola\n");
    }

    #[test]
    fn captures_stderr_of_a_failing_command() {
        let out = run_shell("echo aviso >&2", Shell::Posix);
        assert!(out.success);
        assert_eq!(out.stdout, "");
        assert_eq!(out.stderr, "aviso\n");
    }

    #[test]
    fn reports_a_non_zero_exit_status() {
        let out = run_shell("exit 3", Shell::Posix);
        assert!(!out.success);
        assert!(out.status.contains('3'), "status was {:?}", out.status);
    }
}
