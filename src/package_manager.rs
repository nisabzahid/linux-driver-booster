use crate::model::{DistributionFamily, PackageManager, PackageUpdate};
use crate::process::{run, ProcessError};
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PackageError {
    #[error("package manager command failed: {0}")]
    Process(#[from] ProcessError),
}

pub fn detect(family: &DistributionFamily) -> PackageManager {
    let preferred = match family {
        DistributionFamily::Debian => ["apt", "dnf", "pacman", "zypper"],
        DistributionFamily::Fedora => ["dnf", "apt", "pacman", "zypper"],
        DistributionFamily::Arch => ["pacman", "apt", "dnf", "zypper"],
        DistributionFamily::OpenSuse => ["zypper", "apt", "dnf", "pacman"],
        DistributionFamily::Unknown => ["apt", "dnf", "pacman", "zypper"],
    };
    preferred
        .iter()
        .find(|command| command_exists(command))
        .map(|command| match *command {
            "apt" => PackageManager::Apt,
            "dnf" => PackageManager::Dnf,
            "pacman" => PackageManager::Pacman,
            "zypper" => PackageManager::Zypper,
            _ => PackageManager::Unknown,
        })
        .unwrap_or(PackageManager::Unknown)
}

pub fn check_updates(manager: &PackageManager) -> Result<Vec<PackageUpdate>, PackageError> {
    match manager {
        PackageManager::Apt => Ok(parse_apt(&run("apt", &["list", "--upgradable"])?)),
        PackageManager::Dnf => Ok(parse_dnf(&run_allowing_exit(
            "dnf",
            &["check-update", "--refresh"],
            &[100],
        )?)),
        PackageManager::Pacman => Ok(parse_pacman(&run("pacman", &["-Qu"])?)),
        PackageManager::Zypper => Ok(parse_zypper(&run(
            "zypper",
            &["--non-interactive", "list-updates"],
        )?)),
        PackageManager::Unknown => Ok(Vec::new()),
    }
}

fn run_allowing_exit(
    command: &str,
    args: &[&str],
    allowed: &[i32],
) -> Result<String, PackageError> {
    match run(command, args) {
        Ok(output) => Ok(output),
        Err(ProcessError::Failed {
            code: Some(code),
            stdout,
            stderr,
            ..
        }) if allowed.contains(&code) => Ok(format!("{stdout}\n{stderr}")),
        Err(error) => Err(error.into()),
    }
}

fn parse_apt(output: &str) -> Vec<PackageUpdate> {
    output
        .lines()
        .skip(1)
        .filter_map(|line| {
            let (package, rest) = line.split_once('/')?;
            let candidate = rest.split_whitespace().nth(1)?.to_owned();
            let installed = rest
                .split("[upgradable from:")
                .nth(1)
                .map(|version| version.trim_end_matches(']').trim().to_owned());
            Some(PackageUpdate {
                package: package.to_owned(),
                installed_version: installed,
                candidate_version: Some(candidate),
                source: "APT repository".into(),
            })
        })
        .collect()
}

fn parse_dnf(output: &str) -> Vec<PackageUpdate> {
    output
        .lines()
        .filter_map(|line| {
            let columns: Vec<_> = line.split_whitespace().collect();
            if columns.len() >= 2
                && columns[0] != "Last"
                && columns[0] != "Obsoleting"
                && columns[0] != "Security"
            {
                Some(PackageUpdate {
                    package: columns[0].to_owned(),
                    installed_version: None,
                    candidate_version: Some(columns[1].to_owned()),
                    source: "DNF repository".into(),
                })
            } else {
                None
            }
        })
        .collect()
}

fn parse_pacman(output: &str) -> Vec<PackageUpdate> {
    output
        .lines()
        .filter_map(|line| {
            let (package, versions) = line.split_once(' ')?;
            let (installed, candidate) = versions.trim().split_once(" -> ")?;
            Some(PackageUpdate {
                package: package.to_owned(),
                installed_version: Some(installed.to_owned()),
                candidate_version: Some(candidate.to_owned()),
                source: "Pacman repository".into(),
            })
        })
        .collect()
}

fn parse_zypper(output: &str) -> Vec<PackageUpdate> {
    output
        .lines()
        .filter(|line| line.starts_with("v |"))
        .filter_map(|line| {
            let columns: Vec<_> = line.split('|').map(str::trim).collect();
            (columns.len() >= 5).then(|| PackageUpdate {
                package: columns[1].to_owned(),
                installed_version: Some(columns[2].to_owned()),
                candidate_version: Some(columns[3].to_owned()),
                source: columns[4].to_owned(),
            })
        })
        .collect()
}

fn command_exists(command: &str) -> bool {
    std::env::var_os("PATH")
        .into_iter()
        .flat_map(|path| std::env::split_paths(&path).collect::<Vec<_>>())
        .map(|directory| directory.join(command))
        .any(|path| Path::new(&path).is_file())
}

#[cfg(test)]
mod tests {
    use super::{parse_apt, parse_pacman};

    #[test]
    fn parses_apt_update_line() {
        let updates = parse_apt(
            "Listing...\nlinux-image/jammy-updates 6.8.0 amd64 [upgradable from: 6.5.0]\n",
        );
        assert_eq!(updates[0].package, "linux-image");
        assert_eq!(updates[0].candidate_version.as_deref(), Some("6.8.0"));
    }

    #[test]
    fn parses_pacman_version_transition() {
        let updates = parse_pacman("linux 6.9.1 -> 6.9.2\n");
        assert_eq!(updates[0].installed_version.as_deref(), Some("6.9.1"));
    }
}
