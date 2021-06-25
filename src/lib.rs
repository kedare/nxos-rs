use anyhow::Error;
use serde_json::Value;
use std::process::{Command, ExitStatus};

/// Absolute path to the VSH binary
const VSH_BIN: &str = "/isan/bin/vsh";

/// Represents a command result
pub struct CommandResult {
    stdout: String,
    stderr: String,
    status: ExitStatus,
}

impl CommandResult {
    pub fn json(&self) -> Result<Value, Error> {
        Ok(serde_json::from_str(self.stdout.as_str())?)
    }
}

/// Run the specified command and return the corresponding CommandResult or an Error
pub fn run(command: String) -> Result<CommandResult, Error> {
    let base_command = VSH_BIN;
    let args = vec!["-N", "-c", command.as_str()];

    let output = Command::new(base_command).args(args).output().unwrap();

    Ok(CommandResult {
        stdout: String::from_utf8(output.stdout).unwrap(),
        stderr: String::from_utf8(output.stderr).unwrap(),
        status: output.status,
    })
}
