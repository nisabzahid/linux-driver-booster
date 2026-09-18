use crate::model::{ScanReport, SystemInfo};
use crate::{distro, hardware, package_manager};
use std::process::Command;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScanError {
    #[error(transparent)]
    Distro(#[from] distro::DistroError),
    #[error(transparent)]
    Hardware(#[from] hardware::HardwareError),
    #[error("could not determine the running kernel: {0}")]
    Kernel(#[source] std::io::Error),
    #[error("could not read loaded modules: {0}")]
    Modules(#[source] std::io::Error),
}

#[derive(Debug, Default)]
pub struct SystemScanner;

impl SystemScanner {
    pub fn scan(&self) -> Result<ScanReport, ScanError> {
        let distribution = distro::detect()?;
        let package_manager = package_manager::detect(&distribution.family);
        let kernel = command_output("uname", &["-r"]).map_err(ScanError::Kernel)?;
        let architecture = command_output("uname", &["-m"]).map_err(ScanError::Kernel)?;
        let mut warnings = Vec::new();
        let package_updates = match package_manager::check_updates(&package_manager) {
            Ok(updates) => updates,
            Err(error) => {
                warnings.push(format!("Could not check package updates: {error}"));
                Vec::new()
            }
        };
        let firmware = crate::firmware::check();
        let dkms_modules = crate::dkms::detect();
        let kernels = crate::kernel::status(&kernel, &package_updates);
        Ok(ScanReport {
            system: SystemInfo {
                distribution,
                kernel: kernel.clone(),
                architecture,
                package_manager,
            },
            devices: hardware::detect_pci_devices()?,
            loaded_modules: hardware::loaded_modules().map_err(ScanError::Modules)?,
            package_updates,
            firmware,
            dkms_modules,
            kernels,
            reboot_required: crate::reboot::required(),
            warnings,
        })
    }
}

fn command_output(command: &str, args: &[&str]) -> Result<String, std::io::Error> {
    let output = Command::new(command).args(args).output()?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
    } else {
        Err(std::io::Error::other(format!(
            "{command} exited with {}",
            output.status
        )))
    }
}
