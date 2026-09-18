use crate::model::{KernelStatus, PackageUpdate};
use std::fs;

pub fn status(running: &str, updates: &[PackageUpdate]) -> KernelStatus {
    let installed = fs::read_dir("/boot")
        .ok()
        .into_iter()
        .flat_map(|entries| entries.flatten())
        .filter_map(|entry| entry.file_name().into_string().ok())
        .filter(|name| name.starts_with("vmlinuz-"))
        .map(|name| name.trim_start_matches("vmlinuz-").to_owned())
        .collect();
    let updates = updates
        .iter()
        .filter(|update| {
            update.package.contains("linux-image") || update.package.contains("kernel")
        })
        .cloned()
        .collect();
    KernelStatus {
        running: running.to_owned(),
        installed,
        updates,
    }
}
