use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PrivilegedOperation {
    RefreshRepositories,
    InstallDriver { package_id: String },
    UpdateDriver { package_id: String },
    InstallFirmwareUpdate { device_id: String },
    RepairDkmsModule { module_id: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OperationPreview {
    pub operation: PrivilegedOperation,
    pub affected_packages: Vec<String>,
    pub download_bytes: Option<u64>,
    pub reboot_required: bool,
}

pub fn validate_package_id(package_id: &str) -> bool {
    !package_id.is_empty()
        && package_id.len() <= 255
        && package_id
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || ".:+_@-".contains(character))
}

#[cfg(test)]
mod tests {
    use super::validate_package_id;

    #[test]
    fn rejects_shell_syntax_and_paths() {
        assert!(validate_package_id("nvidia-driver-550"));
        assert!(!validate_package_id("../../run-as-root"));
        assert!(!validate_package_id("package;shutdown"));
    }
}
