extern crate gtk;

use self::gtk::prelude::*;
// use self::gtk::Type::*;
use self::gtk::{Window, WindowType};

pub fn initialize_gtk() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
}

pub fn run_gtk() {
    initialize_gtk();
    let window = Window::new(WindowType::Toplevel);

    window.show_all();

    // Handle closing of the window.
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    // Run the main loop.
    gtk::main();
}
