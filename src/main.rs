use data::core::plugin;
use gtk::{glib, Application};
use gtk::prelude::*;

const APP_ID: &str = "Beth_Editor_GUI";

mod ui;
mod buffer;
mod data;

fn main() {
    let plugin = match buffer::bytebuffer_in::ByteBufferIn::load("data/Fallout4.esm") {
        Ok(mut buffer) => plugin::Plugin::read(&mut buffer),
        Err(err) => {
            println!("Failed to load file: {:?}", err);
            return;
        }
    };

    let plugin_unwrapped = plugin.unwrap();

    println!("{:?}", plugin_unwrapped.groups[0].records[0]);
    println!("{:?}", plugin_unwrapped.groups[0].records.len());
}

/*fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(ui::windows::main_window::build_ui);
    app.run()
}*/