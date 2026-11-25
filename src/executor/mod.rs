//! Run external programs and capture their output.
//!
//! [`CommandExecutor`] is the piece a command wrapper uses before it picks
//! an animation. [`CommandResult`] keeps the original stdout, stderr, and
//! exit code.

use anyhow::{Context, Result};
use std::process::{Command, Output, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;

/// Output of a finished command.
#[derive(Debug, Clone)]
pub struct CommandResult {
    /// Standard output, decoded lossily as UTF-8.
    pub stdout: String,
    /// Standard error, decoded lossily as UTF-8.
    pub stderr: String,
    /// Process exit code, or `-1` when the operating system reported none.
    pub exit_code: i32,
    /// `true` when the process exited with status zero.
    pub success: bool,
}

impl CommandResult {
    /// Create from std::process::Output
    fn from_output(output: Output) -> Self {
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let exit_code = output.status.code().unwrap_or(-1);
        let success = output.status.success();

        Self {
            stdout,
            stderr,
            exit_code,
            success,
        }
    }

    /// Get combined output (stdout + stderr)
    pub fn combined_output(&self) -> String {
        let mut result = String::new();
        if !self.stdout.is_empty() {
            result.push_str(&self.stdout);
        }
        if !self.stderr.is_empty() {
            if !result.is_empty() {
                result.push('\n');
            }
            result.push_str(&self.stderr);
        }
        result
    }
}

/// A command running on a background thread.
pub struct CommandHandle {
    handle: Option<thread::JoinHandle<()>>,
    result: Arc<Mutex<Option<Result<CommandResult>>>>,
}

impl CommandHandle {
    /// Check if the command has completed
    pub fn is_done(&self) -> bool {
        let result = self.result.lock().unwrap();
        result.is_some()
    }

    /// Wait for the command to complete and return the result
    pub fn wait(mut self) -> Result<CommandResult> {
        if let Some(handle) = self.handle.take() {
            handle.join().expect("Command thread panicked");
        }

        let mut result = self.result.lock().unwrap();
        result.take().expect("Command result not available")
    }
}

/// A program plus arguments, ready to run.
pub struct CommandExecutor {
    program: String,
    args: Vec<String>,
    cwd: Option<String>,
}

impl CommandExecutor {
    /// Create a new command executor
    ///
    /// # Arguments
    /// * `program` - The program to execute (e.g., "git", "cargo")
    /// * `args` - Arguments to pass to the program
    ///
    /// # Example
    /// ```
    /// use crabcrust::executor::CommandExecutor;
    ///
    /// let executor = CommandExecutor::new("git", &["status"]);
    /// ```
    pub fn new(program: &str, args: &[&str]) -> Self {
        Self {
            program: program.to_string(),
            args: args.iter().map(|s| s.to_string()).collect(),
            cwd: None,
        }
    }

    /// Set working directory for command execution
    pub fn with_cwd(mut self, cwd: &str) -> Self {
        self.cwd = Some(cwd.to_string());
        self
    }

    /// Execute the command and return the result
    pub fn run(&self) -> Result<CommandResult> {
        let mut cmd = Command::new(&self.program);
        cmd.args(&self.args);

        if let Some(cwd) = &self.cwd {
            cmd.current_dir(cwd);
        }

        // Capture stdout and stderr
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let output = cmd.output().with_context(|| {
            format!(
                "Failed to execute command: {} {:?}",
                self.program, self.args
            )
        })?;

        Ok(CommandResult::from_output(output))
    }

    /// Execute the command in a background thread
    /// Returns a handle that can be used to check completion status
    pub fn run_concurrent(&self) -> CommandHandle {
        let program = self.program.clone();
        let args = self.args.clone();
        let cwd = self.cwd.clone();

        let result = Arc::new(Mutex::new(None));
        let result_clone = Arc::clone(&result);

        let handle = thread::spawn(move || {
            let mut cmd = Command::new(&program);
            cmd.args(&args);

            if let Some(cwd) = cwd {
                cmd.current_dir(cwd);
            }

            cmd.stdout(Stdio::piped());
            cmd.stderr(Stdio::piped());

            let output = cmd
                .output()
                .with_context(|| format!("Failed to execute command: {} {:?}", program, args));

            let cmd_result = match output {
                Ok(output) => Ok(CommandResult::from_output(output)),
                Err(e) => Err(e),
            };

            let mut result_lock = result_clone.lock().unwrap();
            *result_lock = Some(cmd_result);
        });

        CommandHandle {
            handle: Some(handle),
            result,
        }
    }

    /// Execute the command asynchronously
    pub async fn run_async(&self) -> Result<CommandResult> {
        let program = self.program.clone();
        let args = self.args.clone();
        let cwd = self.cwd.clone();

        tokio::task::spawn_blocking(move || {
            let mut cmd = Command::new(&program);
            cmd.args(&args);

            if let Some(cwd) = cwd {
                cmd.current_dir(cwd);
            }

            cmd.stdout(Stdio::piped());
            cmd.stderr(Stdio::piped());

            let output = cmd
                .output()
                .with_context(|| format!("Failed to execute command: {} {:?}", program, args))?;

            Ok(CommandResult::from_output(output))
        })
        .await
        .context("Failed to join async task")?
    }

    /// Get the command as a string for display
    pub fn command_string(&self) -> String {
        format!("{} {}", self.program, self.args.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_executor_creation() {
        let executor = CommandExecutor::new("echo", &["hello"]);
        assert_eq!(executor.program, "echo");
        assert_eq!(executor.args, vec!["hello"]);
    }

    #[test]
    fn test_command_execution() {
        let executor = CommandExecutor::new("echo", &["hello", "world"]);
        let result = executor.run().expect("Failed to run echo");

        assert!(result.success);
        assert!(result.stdout.contains("hello world"));
        assert_eq!(result.exit_code, 0);
    }

    #[test]
    fn test_command_string() {
        let executor = CommandExecutor::new("git", &["commit", "-m", "test"]);
        assert_eq!(executor.command_string(), "git commit -m test");
    }

    #[test]
    fn test_combined_output() {
        let result = CommandResult {
            stdout: "output".to_string(),
            stderr: "error".to_string(),
            exit_code: 0,
            success: true,
        };

        let combined = result.combined_output();
        assert!(combined.contains("output"));
        assert!(combined.contains("error"));
    }

    #[test]
    fn combined_output_skips_empty_streams() {
        let only_out = CommandResult {
            stdout: "out".to_string(),
            stderr: String::new(),
            exit_code: 0,
            success: true,
        };
        assert_eq!(only_out.combined_output(), "out");

        let only_err = CommandResult {
            stdout: String::new(),
            stderr: "err".to_string(),
            exit_code: 1,
            success: false,
        };
        assert_eq!(only_err.combined_output(), "err");
    }

    #[test]
    fn working_directory_is_honored() {
        let result = CommandExecutor::new("pwd", &[])
            .with_cwd("/tmp")
            .run()
            .expect("pwd");
        assert!(result.success);
        assert!(result.stdout.contains("/tmp"));
    }

    #[test]
    fn nonzero_exit_is_reported() {
        let result = CommandExecutor::new("sh", &["-c", "echo out; echo err >&2; exit 3"])
            .run()
            .expect("sh");
        assert!(!result.success);
        assert_eq!(result.exit_code, 3);
        assert!(result.stdout.contains("out"));
        assert!(result.stderr.contains("err"));
        assert!(result.combined_output().contains("err"));
    }

    #[test]
    fn concurrent_echo_completes() {
        let handle = CommandExecutor::new("echo", &["background"]).run_concurrent();
        let result = handle.wait().expect("join");
        assert!(result.success);
        assert!(result.stdout.contains("background"));
    }

    #[test]
    fn async_echo_completes() {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("runtime");
        let result = runtime.block_on(async {
            CommandExecutor::new("echo", &["async"])
                .run_async()
                .await
                .expect("echo")
        });
        assert!(result.stdout.contains("async"));
    }
}
