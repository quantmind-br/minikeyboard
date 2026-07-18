//! Mini Keyboard — native Linux programmer entry point.

use gtk::prelude::*;
use gtk::{gio, glib};
use tracing_subscriber::EnvFilter;

use minikeyboard::ui::MiniKeyboardWindow;
use minikeyboard::APP_ID;

fn main() -> glib::ExitCode {
    // English UI (product decision). Set before GTK init.
    // SAFETY: single-threaded startup, no other threads yet.
    unsafe {
        std::env::set_var("LC_ALL", "C.UTF-8");
        std::env::set_var("LANGUAGE", "en_US");
    }

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("minikeyboard=info"));
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(true)
        .init();

    gio::resources_register_include!("minikeyboard.gresource")
        .expect("register minikeyboard.gresource");

    let app = adw::Application::builder().application_id(APP_ID).build();

    app.connect_activate(|app| {
        let win = MiniKeyboardWindow::new(app);
        win.window.present();
    });

    app.run()
}
