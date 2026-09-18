use std::process::Command;

pub fn required() -> bool {
    ["/run/reboot-required", "/var/run/reboot-required"]
        .iter()
        .any(|path| std::path::Path::new(path).exists())
        || needs_restarting()
}

fn needs_restarting() -> bool {
    let Ok(output) = Command::new("needs-restarting").args(["-r"]).output() else {
        return false;
    };
    output.status.code() == Some(1)
}
