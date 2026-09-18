use crate::model::{Distribution, DistributionFamily};
use std::collections::HashMap;
use std::fs;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DistroError {
    #[error("could not read /etc/os-release: {0}")]
    Read(#[from] std::io::Error),
}

pub fn detect() -> Result<Distribution, DistroError> {
    detect_from(&fs::read_to_string("/etc/os-release")?)
}

pub fn detect_from(contents: &str) -> Result<Distribution, DistroError> {
    let values: HashMap<_, _> = contents
        .lines()
        .filter_map(|line| line.split_once('='))
        .map(|(key, value)| (key, value.trim_matches('"').to_owned()))
        .collect();
    let id = values
        .get("ID")
        .cloned()
        .unwrap_or_else(|| "unknown".into());
    let family = match id.as_str() {
        "debian" | "ubuntu" | "linuxmint" | "pop" => DistributionFamily::Debian,
        "fedora" | "nobara" => DistributionFamily::Fedora,
        "arch" | "endeavouros" | "manjaro" => DistributionFamily::Arch,
        "opensuse" | "opensuse-tumbleweed" | "opensuse-leap" => DistributionFamily::OpenSuse,
        _ => DistributionFamily::Unknown,
    };
    Ok(Distribution {
        id,
        name: values
            .get("PRETTY_NAME")
            .cloned()
            .unwrap_or_else(|| "Unknown Linux distribution".into()),
        version: values.get("VERSION_ID").cloned(),
        family,
    })
}

#[cfg(test)]
mod tests {
    use super::detect_from;
    use crate::model::DistributionFamily;

    #[test]
    fn detects_ubuntu_as_debian_family() {
        let distro =
            detect_from("ID=ubuntu\nPRETTY_NAME=\"Ubuntu 24.04 LTS\"\nVERSION_ID=\"24.04\"\n")
                .unwrap();
        assert_eq!(distro.family, DistributionFamily::Debian);
        assert_eq!(distro.version.as_deref(), Some("24.04"));
    }

    #[test]
    fn unknown_distribution_is_not_guessed() {
        let distro = detect_from("ID=customlinux\n").unwrap();
        assert_eq!(distro.family, DistributionFamily::Unknown);
    }
}
