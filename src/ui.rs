use adw::prelude::*;
use adw::{Application, ApplicationWindow, HeaderBar, StatusPage};
use gtk4::glib;

pub fn run() {
    let application = Application::builder()
        .application_id("com.linuxdriverbooster.DriverBooster")
        .build();
    application.connect_activate(build_window);
    application.run();
}

fn build_window(application: &Application) {
    let status = StatusPage::builder()
        .title("Linux Driver Booster")
        .description("Scan your system for supported hardware and loaded Linux drivers.")
        .icon_name("computer-symbolic")
        .build();
    let scan_button = gtk4::Button::with_label("Scan Now");
    scan_button.add_css_class("suggested-action");
    let result_label = gtk4::Label::new(Some("No scan has been run yet."));
    result_label.set_wrap(true);
    let result_label_for_callback = result_label.clone();
    scan_button.connect_clicked(move |_| {
        let result_label = result_label_for_callback.clone();
        std::thread::spawn(move || {
            let result = crate::SystemScanner.scan();
            glib::MainContext::default().invoke(move || match result {
                Ok(report) => result_label.set_text(&format!(
                    "{} devices scanned\n{} package updates\n{} firmware updates\n{} DKMS modules\nReboot required: {}",
                    report.devices.len(),
                    report.package_updates.len(),
                    report.firmware.updates.len(),
                    report.dkms_modules.len(),
                    if report.reboot_required { "yes" } else { "no" }
                )),
                Err(error) => result_label.set_text(&format!("Scan failed: {error}")),
            });
        });
    });
    let content = gtk4::Box::new(gtk4::Orientation::Vertical, 12);
    content.append(&scan_button);
    content.append(&result_label);
    status.set_child(Some(&content));

    let window = ApplicationWindow::builder()
        .application(application)
        .title("Linux Driver Booster")
        .default_width(760)
        .default_height(520)
        .content(&status)
        .build();
    let header = HeaderBar::new();
    window.set_titlebar(Some(&header));
    window.present();
}
