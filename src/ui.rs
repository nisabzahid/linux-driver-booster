use adw::prelude::*;
use adw::{Application, ApplicationWindow, StatusPage};
use gtk4::glib;
use std::sync::mpsc;
use std::time::Duration;

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
        let (sender, receiver) = mpsc::channel();
        std::thread::spawn(move || {
            let result = crate::SystemScanner.scan();
            let message = match result {
                Ok(report) => format!(
                    "{} devices scanned\n{} package updates\n{} firmware updates\n{} DKMS modules\nReboot required: {}",
                    report.devices.len(),
                    report.package_updates.len(),
                    report.firmware.updates.len(),
                    report.dkms_modules.len(),
                    if report.reboot_required { "yes" } else { "no" }
                ),
                Err(error) => format!("Scan failed: {error}"),
            };
            let _ = sender.send(message);
        });
        glib::timeout_add_local(Duration::from_millis(100), move || {
            match receiver.try_recv() {
                Ok(message) => {
                    result_label.set_text(&message);
                    glib::ControlFlow::Break
                }
                Err(mpsc::TryRecvError::Empty) => glib::ControlFlow::Continue,
                Err(mpsc::TryRecvError::Disconnected) => {
                    result_label.set_text("Scan worker stopped unexpectedly.");
                    glib::ControlFlow::Break
                }
            }
        });
    });
    let window = ApplicationWindow::builder()
        .application(application)
        .title("Linux Driver Booster")
        .default_width(760)
        .default_height(520)
        .content(&status)
        .build();
    let content = gtk4::Box::new(gtk4::Orientation::Vertical, 12);
    let toolbar = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
    let close_button = gtk4::Button::from_icon_name("window-close-symbolic");
    close_button.set_tooltip_text(Some("Close"));
    let window_for_close = window.clone();
    close_button.connect_clicked(move |_| window_for_close.close());
    toolbar.append(&close_button);
    content.append(&toolbar);
    content.append(&scan_button);
    content.append(&result_label);
    status.set_child(Some(&content));
    window.present();
}
