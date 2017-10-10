extern crate gtk;
use self::gtk::prelude::*;
use self::gtk::*;

pub fn make_scrollable(widget: &Widget) -> ScrolledWindow {
    let scrolled_window = gtk::ScrolledWindow::new(None, None);
    scrolled_window.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    scrolled_window.add(widget);
    scrolled_window
}

pub fn initialize_gtk() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
}
