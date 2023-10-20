use gtk::prelude::*;

fn main() {
    let application =
        gtk::Application::new(Some("com.github.keens.gtk-examples.basic"), Default::default());

    application.connect_activate(build_ui);

    application.run();
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title(Some("First GTK Program"));
    window.set_default_size(350, 70);

    let vbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)

        .halign(gtk::Align::Start)

        .spacing(6)

        .margin_bottom(6)
        .margin_top(6)
        .margin_start(6)
        .margin_end(6)

        .build();

    window.set_child(Some(&vbox));

    vbox.append(&build_button());
    vbox.append(&build_scale());
    vbox.append(&build_switch());
    vbox.append(&build_password_entry());
    vbox.append(&build_frame());

    window.show();
}

fn build_button() -> gtk::Button {
    let button = gtk::Button::with_label("Click me!");
    button.connect_clicked(|_| {
        println!("Clicked!");
    });
    button
}

fn build_scale() -> gtk::Scale {
    let scale = gtk::Scale::builder()
        .draw_value(true)

        .adjustment(
            &gtk::Adjustment::builder()
                .lower(0.0)
                .upper(100.0)
                .value(50.0)
                .step_increment(1.0)
                .page_increment(10.0)
                .build(),
        )

        .digits(0)
        .round_digits(0)
        .build();

    scale.connect_value_changed(|s| {
        println!("value changed: {}", s.value());
    });
    scale
}

fn build_switch() -> gtk::Switch {
    let switch = gtk::Switch::builder().halign(gtk::Align::End).build();
    switch.connect_active_notify(|s| println!("state changed: {:?}", s.is_active()));
    switch
}

fn build_password_entry() -> gtk::PasswordEntry {
    gtk::PasswordEntry::new()
}

fn build_frame() -> gtk::Frame {
    let frame = gtk::Frame::builder()
        .label("Frame")
        .child(&build_switch())
        .build();
    frame
}