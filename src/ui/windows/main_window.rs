use gtk::{glib, prelude::*, ApplicationWindow, Image, Label, Menu, MenuButton, MenuItem};
use gtk::{Application, Button};
use native_dialog::FileDialog;

use crate::buffer::bytebuffer_in as buffer;

use crate::data::core::plugin::Plugin;
use crate::ui::windows::help_window;
use crate::{set_loaded_data, GLOBAL_PLUGIN};

use super::object_window;

pub fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .default_height(600)
        .default_width(500)
        .title("Open-Rust-CK")
        .build();

    let box_container = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(1)
        .margin_start(1)
        .margin_end(1)
        .margin_top(1)
        .margin_bottom(1)
        .height_request(300)
        .width_request(500)
        .build();

    box_container.set_child(Some(&top_toolbar(app)));
    box_container.set_child(Some(&icon_toolbar()));

    object_window::build_ui();

    window.set_child(Some(&box_container));
    window.show_all();
    window.present();
}

fn top_toolbar(app: &Application) -> gtk::Box {
    let toolbar = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(1)
        .hexpand(false)
        .vexpand(false)
        .build();

    let file_button = create_file_button(app);
    toolbar.pack_start(&file_button, false, false, 0);

    let edit_button = Button::builder()
        .label("Edit")
        .hexpand(false)
        .vexpand(false)
        .relief(gtk::ReliefStyle::None)
        .build();
    toolbar.pack_start(&edit_button, false, false, 0);

    let view_button = Button::builder()
        .label("View")
        .hexpand(false)
        .vexpand(false)
        .relief(gtk::ReliefStyle::None)
        .build();
    toolbar.pack_start(&view_button, false, false, 0);

    let world_button = Button::builder()
        .label("World")
        .hexpand(false)
        .vexpand(false)
        .relief(gtk::ReliefStyle::None)
        .build();
    toolbar.pack_start(&world_button, false, false, 0);

    let navmesh_button = Button::builder()
        .label("NavMesh")
        .hexpand(false)
        .vexpand(false)
        .relief(gtk::ReliefStyle::None)
        .build();
    toolbar.pack_start(&navmesh_button, false, false, 0);

    let visibility_button = Button::builder()
        .label("Visibility")
        .hexpand(false)
        .vexpand(false)
        .relief(gtk::ReliefStyle::None)
        .build();
    toolbar.pack_start(&visibility_button, false, false, 0);

    let character_button = Button::builder()
        .label("Character")
        .hexpand(false)
        .vexpand(false)
        .relief(gtk::ReliefStyle::None)
        .build();
    toolbar.pack_start(&character_button, false, false, 0);

    let gameplay_button = Button::builder()
        .label("Gameplay")
        .hexpand(false)
        .vexpand(false)
        .relief(gtk::ReliefStyle::None)
        .build();
    toolbar.pack_start(&gameplay_button, false, false, 0);

    let help_button = create_help_button(app);

    toolbar.pack_start(&help_button, false, false, 0);

    let links_button = create_links_button(app);
    toolbar.pack_start(&links_button, false, false, 0);

    return toolbar;
}

fn icon_toolbar() -> gtk::Box {
    let toolbar = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .hexpand(false)
        .vexpand(false)
        .build();

    let data_image = Image::from_file("./src/ui/icons/folder_open.svg");
    let data_box = gtk::Box::builder().build();
    data_box.pack_start(&data_image, false, false, 0);
    let data_button = Button::builder()
        .hexpand(false)
        .vexpand(false)
        .relief(gtk::ReliefStyle::None)
        .image(&data_image)
        .tooltip_text("Load Master/Plugin Files")
        .build();

    data_button.connect_clicked(|_| {
        let path = FileDialog::new()
            .set_location("~/Downloads")
            .add_filter("Bethesda Plugins", &["esp", "esm", "esl"])
            .show_open_single_file()
            .unwrap();

        let path = match path {
            Some(path) => path,
            None => return,
        };

        let string_path = path.into_os_string().into_string().unwrap();

        let mut plugin = GLOBAL_PLUGIN.lock().unwrap();

        // Create a new Plugin instance and set it with the loaded data
        let loaded_plugin = match buffer::ByteBufferIn::load(&string_path) {
            Ok(mut buffer) => {
                if let Some(loaded_plugin) = Plugin::read(&mut buffer) {
                    loaded_plugin
                } else {
                    // Handle the case when reading fails
                    return;
                }
            }
            Err(err) => {
                println!("Failed to load file: {:?}", err);
                return;
            }
        };

        // Update the global_plugin with the loaded data
        *plugin = Some(loaded_plugin);
        set_loaded_data(true);
    });

    let save_image = Image::from_file("./src/ui/icons/save.svg");
    let save_box = gtk::Box::builder().build();
    save_box.pack_start(&save_image, false, false, 0);
    let save_button = Button::builder()
        .hexpand(false)
        .vexpand(false)
        .relief(gtk::ReliefStyle::None)
        .child(&save_box)
        .tooltip_text("Save Plugin (CTRL+S)")
        .build();

    toolbar.pack_start(&data_button, false, false, 0);
    toolbar.pack_start(&save_button, false, false, 0);

    return toolbar;
}

fn create_help_button(app: &Application) -> Button {
    let help_button = Button::builder()
        .label("Help")
        .hexpand(false)
        .vexpand(false)
        .relief(gtk::ReliefStyle::None)
        .build();

    help_button.connect_clicked(glib::clone!(@weak app => move |_| {
        help_window::build_ui();
    }));

    help_button
}

fn create_links_button(app: &Application) -> gtk::MenuButton {
    let link_label = Label::builder().label("Links").build();
    let link_box = gtk::Box::builder().build();
    link_box.pack_start(&link_label, false, false, 0);

    let links_button = MenuButton::builder()
        .relief(gtk::ReliefStyle::None)
        .child(&link_box)
        .build();

    let links_menu = Menu::builder().build();

    let creation_kit_wiki = MenuItem::builder().label("Creation Kit Wiki").build();
    creation_kit_wiki.connect_activate(glib::clone!(@weak app => move |_| {
        let _ = open::that("https://www.creationkit.com/fallout4/index.php?title=Main_Page");
    }));

    let fallout_cascadia_wiki = MenuItem::builder().label("Fallout Cascadia Wiki").build();
    fallout_cascadia_wiki.connect_activate(glib::clone!(@weak app => move |_| {
        let _ = open::that("https://docs.falloutcascadia.com");
    }));

    links_menu.append(&creation_kit_wiki);
    links_menu.append(&fallout_cascadia_wiki);
    links_menu.show_all();

    links_button.set_popup(Some(&links_menu));

    links_button
}

fn create_file_button(app: &Application) -> gtk::MenuButton {
    let file_label = Label::builder().label("File").build();
    let file_box = gtk::Box::builder().build();
    file_box.pack_start(&file_label, false, false, 0);

    let file_button = MenuButton::builder()
        .relief(gtk::ReliefStyle::None)
        .child(&file_box)
        .build();

    let file_menu = Menu::builder().build();

    let data = MenuItem::builder().label("Data...").build();

    data.connect_activate(|_| {
        let path = FileDialog::new()
            .set_location("~/Downloads")
            .add_filter("Bethesda Plugins", &["esp", "esm", "esl"])
            .show_open_single_file()
            .unwrap();

        let path = match path {
            Some(path) => path,
            None => return,
        };

        let string_path = path.into_os_string().into_string().unwrap();

        let mut plugin = GLOBAL_PLUGIN.lock().unwrap();

        // Create a new Plugin instance and set it with the loaded data
        let loaded_plugin = match buffer::ByteBufferIn::load(&string_path) {
            Ok(mut buffer) => {
                if let Some(loaded_plugin) = Plugin::read(&mut buffer) {
                    loaded_plugin
                } else {
                    // Handle the case when reading fails
                    return;
                }
            }
            Err(err) => {
                println!("Failed to load file: {:?}", err);
                return;
            }
        };

        // Update the global_plugin with the loaded data
        *plugin = Some(loaded_plugin);
        set_loaded_data(true);
    });

    let exit = MenuItem::builder().label("Exit").build();
    exit.connect_activate(|_| {
        // TODO - check for unsaved edits.
        let _ = std::process::exit(0);
    });

    file_menu.append(&data);
    file_menu.append(&exit);

    file_menu.show_all();

    file_button.set_popup(Some(&file_menu));

    file_button
}
