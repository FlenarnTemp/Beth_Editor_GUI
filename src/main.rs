use data::core::plugin;
use gtk::{glib, Application};
use gtk::prelude::*;

use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;

use crate::data::core::plugin::Plugin;

const APP_ID: &str = "Beth_Editor_GUI";

mod ui;
mod buffer;
mod data;

lazy_static! {
    static ref GLOBAL_PLUGIN: Arc<Mutex<Option<Plugin>>> = Arc::new(Mutex::new(None));
}

static mut LOADED_DATA: bool = false;

pub fn check_loaded_data() -> bool {
    unsafe {
        LOADED_DATA
    }
}

pub fn set_loaded_data(value: bool) {
    unsafe {
        LOADED_DATA = value;
    }
}

/*fn main() {
    println!("{}", format!("{:x}", 1125660))
}*/

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(ui::windows::main_window::build_ui);
    app.run()
}