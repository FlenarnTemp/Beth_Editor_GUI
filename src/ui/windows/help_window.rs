use gtk::{prelude::*, ApplicationWindow, Align, Label};

pub fn build_ui() {
    let window = ApplicationWindow::builder()
        .default_height(300)
        .default_width(100)
        .title("About Open-Rust-CK")
        .resizable(false)
        .build();

    window.set_keep_above(true);

    let help_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(10)
        .margin_start(10)
        .margin_end(10)
        .margin_top(10)
        .margin_bottom(10)
        .build();
    help_box.set_size_request(100, 300);

    let help_name = Label::new(Some(
        "Open-Rust-CK"
    ));
    help_name.set_valign(Align::Center);

    let help_version = Label::new(Some(
        "Version 0.0.1A"
    ));
    help_version.set_valign(Align::Center);

    // TODO - add in latest commit hash.
    let help_latest = Label::new(Some(
        "Latest commit: [TODO]"
    ));

    let help_text = Label::new(Some(
        "Open-Rust-CK is an open-source re-implementation of Bethesda Softworks Creation Kit for Fallout 4.\n Written in Rust using the GTK framework."
    )); 
    help_text.set_valign(Align::Center);
    help_text.set_justify(gtk::Justification::Center);

    help_box.pack_start(&help_name, false, false, 0);
    help_box.pack_start(&help_version, false, false, 0);
    help_box.pack_start(&help_latest, false, false, 0);
    help_box.pack_start(&help_text, false, false, 0);

    window.set_child(Some(&help_box));
    window.show_all();

    window.present();
}