use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HistoryEvent {
    pub timestamp: u64,
    pub operation: String,
    pub target: String,
    pub from_version: Option<String>,
    pub to_version: Option<String>,
    pub result: String,
    pub reboot_required: bool,
}

#[derive(Debug, Error)]
pub enum HistoryError {
    #[error("could not read history: {0}")]
    Read(#[source] std::io::Error),
    #[error("could not write history: {0}")]
    Write(#[source] std::io::Error),
    #[error("invalid history: {0}")]
    Format(#[from] serde_json::Error),
}

pub fn load() -> Result<Vec<HistoryEvent>, HistoryError> {
    let path = path();
    if !path.exists() {
        return Ok(Vec::new());
    }
    let contents = fs::read_to_string(path).map_err(HistoryError::Read)?;
    Ok(serde_json::from_str(&contents)?)
}

pub fn append(mut event: HistoryEvent) -> Result<(), HistoryError> {
    let mut events = load()?;
    if event.timestamp == 0 {
        event.timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
    }
    events.push(event);
    let path = path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(HistoryError::Write)?;
    }
    fs::write(path, serde_json::to_vec_pretty(&events)?).map_err(HistoryError::Write)
}

fn path() -> PathBuf {
    std::env::var_os("XDG_STATE_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(std::env::var_os("HOME").unwrap_or_default()).join(".local/state")
        })
        .join("linux-driver-booster/history.json")
}
