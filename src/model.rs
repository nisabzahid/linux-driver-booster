use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SystemInfo {
    pub distribution: Distribution,
    pub kernel: String,
    pub architecture: String,
    pub package_manager: PackageManager,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Distribution {
    pub id: String,
    pub name: String,
    pub version: Option<String>,
    pub family: DistributionFamily,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DistributionFamily {
    Debian,
    Fedora,
    Arch,
    OpenSuse,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PackageManager {
    Apt,
    Dnf,
    Pacman,
    Zypper,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Device {
    pub name: String,
    pub vendor: Option<String>,
    pub device_id: Option<String>,
    pub bus: String,
    pub class: String,
    pub kernel_driver: Option<String>,
    pub driver_version: Option<String>,
    pub firmware_version: Option<String>,
    pub status: DriverStatus,
    pub recommendation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DriverStatus {
    DriverLoaded,
    MissingDriver,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ScanReport {
    pub system: SystemInfo,
    pub devices: Vec<Device>,
    pub loaded_modules: Vec<String>,
    pub package_updates: Vec<PackageUpdate>,
    pub firmware: FirmwareStatus,
    pub dkms_modules: Vec<DkmsModule>,
    pub kernels: KernelStatus,
    pub reboot_required: bool,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PackageUpdate {
    pub package: String,
    pub installed_version: Option<String>,
    pub candidate_version: Option<String>,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FirmwareStatus {
    pub tool_available: bool,
    pub updates: Vec<FirmwareUpdate>,
    pub checked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FirmwareUpdate {
    pub device: String,
    pub current_version: Option<String>,
    pub available_version: Option<String>,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DkmsModule {
    pub module: String,
    pub version: Option<String>,
    pub kernel: Option<String>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct KernelStatus {
    pub running: String,
    pub installed: Vec<String>,
    pub updates: Vec<PackageUpdate>,
}
