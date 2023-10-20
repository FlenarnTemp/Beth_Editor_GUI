use gtk::{glib, Application};
use gtk::prelude::*;

const APP_ID: &str = "Beth_Editor_GUI";

mod ui;
mod buffer;

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(ui::windows::main_window::build_ui);
    app.run()
}

