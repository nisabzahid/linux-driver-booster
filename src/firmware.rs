use crate::model::{FirmwareStatus, FirmwareUpdate};
use crate::process::run;

pub fn check() -> FirmwareStatus {
    if !command_exists("fwupdmgr") {
        return FirmwareStatus {
            tool_available: false,
            updates: Vec::new(),
            checked: false,
        };
    }
    let output = match run("fwupdmgr", &["get-updates"]) {
        Ok(output) => output,
        Err(_) => {
            return FirmwareStatus {
                tool_available: true,
                updates: Vec::new(),
                checked: true,
            }
        }
    };
    FirmwareStatus {
        tool_available: true,
        updates: parse_updates(&output),
        checked: true,
    }
}

fn parse_updates(output: &str) -> Vec<FirmwareUpdate> {
    output
        .lines()
        .filter_map(|line| {
            let (device, summary) = line.split_once(" has firmware updates:")?;
            Some(FirmwareUpdate {
                device: device.trim().to_owned(),
                current_version: None,
                available_version: None,
                summary: summary.trim().to_owned(),
            })
        })
        .collect()
}

fn command_exists(command: &str) -> bool {
    std::env::var_os("PATH")
        .into_iter()
        .flat_map(|path| std::env::split_paths(&path).collect::<Vec<_>>())
        .any(|directory| directory.join(command).is_file())
}
