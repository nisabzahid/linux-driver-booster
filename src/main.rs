#[cfg(not(feature = "gtk-ui"))]
use linux_driver_booster::SystemScanner;

#[cfg(feature = "gtk-ui")]
fn main() {
    linux_driver_booster::ui::run();
}

#[cfg(not(feature = "gtk-ui"))]
fn main() {
    let arguments: Vec<_> = std::env::args().collect();
    if arguments.iter().any(|argument| argument == "--history") {
        match linux_driver_booster::history::load() {
            Ok(events) => println!(
                "{}",
                serde_json::to_string_pretty(&events).expect("history is serializable")
            ),
            Err(error) => {
                eprintln!("Could not read history: {error}");
                std::process::exit(1);
            }
        }
        return;
    }
    let json = arguments.iter().any(|argument| argument == "--json");
    match SystemScanner.scan() {
        Ok(report) if json => println!(
            "{}",
            serde_json::to_string_pretty(&report).expect("report is serializable")
        ),
        Ok(report) => print_report(&report),
        Err(error) => {
            eprintln!("Scan failed: {error}");
            std::process::exit(1);
        }
    }
}

#[cfg(not(feature = "gtk-ui"))]
fn print_report(report: &linux_driver_booster::model::ScanReport) {
    println!("Linux Driver Booster 0.1");
    println!();
    println!("OS: {}", report.system.distribution.name);
    println!("Kernel: {}", report.system.kernel);
    println!("Architecture: {}", report.system.architecture);
    println!("Package manager: {:?}", report.system.package_manager);
    println!("Updates available: {}", report.package_updates.len());
    println!("Firmware updates: {}", report.firmware.updates.len());
    println!("Running kernel: {}", report.kernels.running);
    println!("Installed kernels: {}", report.kernels.installed.len());
    println!("DKMS modules: {}", report.dkms_modules.len());
    println!(
        "Reboot required: {}",
        if report.reboot_required { "yes" } else { "no" }
    );
    println!();
    println!("Hardware:");
    for device in &report.devices {
        let marker = match device.status {
            linux_driver_booster::model::DriverStatus::DriverLoaded => "OK",
            linux_driver_booster::model::DriverStatus::MissingDriver => "!!",
            linux_driver_booster::model::DriverStatus::Unknown => "??",
        };
        println!(
            "[{marker}] {} ({})",
            device.name,
            device.kernel_driver.as_deref().unwrap_or("no driver")
        );
    }
    println!();
    println!("Loaded kernel modules: {}", report.loaded_modules.len());
    for update in &report.package_updates {
        println!(
            "[UPDATE] {} -> {}",
            update.package,
            update.candidate_version.as_deref().unwrap_or("unknown")
        );
    }
    for warning in &report.warnings {
        println!("[WARN] {warning}");
    }
}
