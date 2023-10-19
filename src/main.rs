use gtk::{glib, Application};
use gtk::prelude::*;

const APP_ID: &str = "org.beth_editor.Open-Rust-CK";

mod ui;

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(ui::windows::main_window::build_ui);
    app.run()
}

