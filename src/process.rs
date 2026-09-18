use std::process::Command;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProcessError {
    #[error("could not start {command}: {source}")]
    Start {
        command: String,
        #[source]
        source: std::io::Error,
    },
    #[error("{command} exited with status {code:?}: {stderr}")]
    Failed {
        command: String,
        code: Option<i32>,
        stdout: String,
        stderr: String,
    },
}

pub fn run(command: &str, args: &[&str]) -> Result<String, ProcessError> {
    let output = Command::new(command)
        .args(args)
        .output()
        .map_err(|source| ProcessError::Start {
            command: command.to_owned(),
            source,
        })?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        Err(ProcessError::Failed {
            command: command.to_owned(),
            code: output.status.code(),
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).trim().to_owned(),
        })
    }
}
