use crate::model::DkmsModule;
use crate::process::run;

pub fn detect() -> Vec<DkmsModule> {
    let Ok(output) = run("dkms", &["status"]) else {
        return Vec::new();
    };
    output.lines().filter_map(parse_line).collect()
}

fn parse_line(line: &str) -> Option<DkmsModule> {
    let (name_version, rest) = line.split_once(',')?;
    let (module, version) = name_version.trim().split_once('/')?;
    let status = rest.trim().to_owned();
    let kernel = status
        .split_once(':')
        .map(|(kernel, _)| kernel.trim().to_owned());
    Some(DkmsModule {
        module: module.to_owned(),
        version: Some(version.to_owned()),
        kernel,
        status,
    })
}

#[cfg(test)]
mod tests {
    use super::parse_line;

    #[test]
    fn parses_dkms_status() {
        let module = parse_line("nvidia/550.1, 6.8.0-1-generic, x86_64: installed").unwrap();
        assert_eq!(module.module, "nvidia");
        assert_eq!(module.version.as_deref(), Some("550.1"));
    }
}
