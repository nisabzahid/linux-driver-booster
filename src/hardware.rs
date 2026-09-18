use crate::model::{Device, DriverStatus};
use std::fs;
use std::process::Command;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HardwareError {
    #[error("lspci is unavailable: {0}")]
    Command(#[from] std::io::Error),
    #[error("lspci failed: {0}")]
    Failed(String),
}

pub fn detect_pci_devices() -> Result<Vec<Device>, HardwareError> {
    let output = Command::new("lspci").args(["-D", "-mm", "-k"]).output()?;
    if !output.status.success() {
        return Err(HardwareError::Failed(
            String::from_utf8_lossy(&output.stderr).trim().into(),
        ));
    }
    Ok(parse_lspci(&String::from_utf8_lossy(&output.stdout)))
}

pub fn loaded_modules() -> Result<Vec<String>, std::io::Error> {
    Ok(fs::read_to_string("/proc/modules")?
        .lines()
        .filter_map(|line| line.split_whitespace().next())
        .map(str::to_owned)
        .collect())
}

fn parse_lspci(output: &str) -> Vec<Device> {
    output
        .lines()
        .filter_map(|record| {
            let mut quoted = record
                .split('"')
                .enumerate()
                .filter_map(|(index, value)| (index % 2 == 1).then_some(value));
            let slot = record.split_whitespace().next()?.to_owned();
            let class = quoted.next()?.to_owned();
            let vendor = quoted.next()?.to_owned();
            let name = quoted.next()?.to_owned();
            let driver = sysfs_driver(&slot);
            Some(Device {
                name,
                vendor: Some(vendor),
                device_id: sysfs_value(&slot, "device"),
                bus: slot,
                class,
                kernel_driver: driver.clone(),
                driver_version: None,
                firmware_version: None,
                status: if driver.is_some() {
                    DriverStatus::DriverLoaded
                } else {
                    DriverStatus::MissingDriver
                },
                recommendation: if driver.is_some() {
                    "The kernel reports a driver loaded; package update checks are not yet enabled."
                        .into()
                } else {
                    "No loaded kernel driver was reported for this device.".into()
                },
            })
        })
        .collect()
}

fn sysfs_driver(slot: &str) -> Option<String> {
    fs::read_link(format!("/sys/bus/pci/devices/{slot}/driver"))
        .ok()
        .and_then(|path| {
            path.file_name()
                .map(|name| name.to_string_lossy().into_owned())
        })
}

fn sysfs_value(slot: &str, field: &str) -> Option<String> {
    fs::read_to_string(format!("/sys/bus/pci/devices/{slot}/{field}"))
        .ok()
        .map(|value| value.trim().to_owned())
}

#[cfg(test)]
mod tests {
    use super::parse_lspci;
    use crate::model::DriverStatus;

    #[test]
    fn parses_machine_readable_pci_record() {
        let devices = parse_lspci(
            "0000:00:02.0 \"VGA compatible controller\" \"Intel Corporation\" \"UHD Graphics\" -p00 \"Intel Corporation\" \"UHD Graphics\"",
        );
        assert_eq!(devices.len(), 1);
        assert_eq!(devices[0].name, "UHD Graphics");
        assert_eq!(devices[0].vendor.as_deref(), Some("Intel Corporation"));
        assert!(matches!(
            devices[0].status,
            DriverStatus::DriverLoaded | DriverStatus::MissingDriver
        ));
    }
}
