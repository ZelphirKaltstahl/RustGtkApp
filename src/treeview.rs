//! # TreeView Sample
//!
//! This sample demonstrates how to create a `TreeView`
//! with either a `ListStore` or `TreeStore`.

extern crate glib;
extern crate gtk;
extern crate gdk_pixbuf;

use gtk::prelude::*;
use gtk::{
    ButtonsType, CellRendererPixbuf, CellRendererText, MessageDialog, MessageType, Orientation,
    TreeStore, TreeView, TreeViewColumn, Window, WindowPosition, WindowType, DIALOG_MODAL
};
use gdk_pixbuf::Pixbuf;

fn append_text_column(tree: &TreeView) {
    let column = TreeViewColumn::new();
    let cell = CellRendererText::new();

    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", 0);
    tree.append_column(&column);
}

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);

    window.set_title("TreeView Sample");
    window.set_position(WindowPosition::Center);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    // left pane

    let left_tree = TreeView::new();
    let left_store = TreeStore::new(&[String::static_type()]);

    left_tree.set_model(Some(&left_store));
    left_tree.set_headers_visible(false);
    append_text_column(&left_tree);

    for i in 0..10 {
        // insert_with_values takes two slices: column indices and ToValue
        // trait objects. ToValue is implemented for strings, numeric types,
        // bool and Object descendants
        let iter = left_store.insert_with_values(None, None, &[0], &[&format!("Hello {}", i)]);

        for _ in 0..i {
            left_store.insert_with_values(Some(&iter), None, &[0], &[&"I'm a child node"]);
        }
    }

    // right pane
    let right_tree = TreeView::new();
    let right_column_types = [Pixbuf::static_type(), String::static_type()];
    let right_store = TreeStore::new(&right_column_types);
    let renderer = CellRendererPixbuf::new();
    let col = TreeViewColumn::new();

    col.set_title("Picture");
    col.pack_start(&renderer, false);

    col.add_attribute(&renderer, "pixbuf", 0);

    let renderer2 = CellRendererText::new();
    col.pack_start(&renderer2, true);
    col.add_attribute(&renderer2, "text", 1);
    let image = Pixbuf::new_from_file("./resources/eye.png").or_else(|err| {
        let mut msg = err.to_string();
        if err.kind() == Some(glib::FileError::Noent) {
            msg.push_str("\nRelaunch this example from the same level \
                          as the `resources` folder");
        }
        let window = window.clone();

        gtk::idle_add(move || {
            let dialog = MessageDialog::new(Some(&window), DIALOG_MODAL,
                MessageType::Error, ButtonsType::Ok, &msg);
            dialog.run();
            dialog.destroy();
            Continue(false)
        });

        Err(())
    }).ok();

    right_tree.append_column(&col);
    right_tree.set_model(Some(&right_store));
    right_tree.set_headers_visible(true);

    for _ in 0..10 {
        right_store.insert_with_values(None, None, &[0, 1],
                                       &[&image, &"I'm a child node with an image"]);
    }

    // selection and path manipulation

    let left_selection = left_tree.get_selection();
    let right_tree1 = right_tree.clone();
    left_selection.connect_changed(move |tree_selection| {
        let (left_model, iter) = tree_selection.get_selected().expect("Couldn't get selected");
        let mut path = left_model.get_path(&iter).expect("Couldn't get path");
        // get the top-level element path
        while path.get_depth() > 1 {
            path.up();
        }
        right_tree1.get_selection().select_path(&path);
    });

    // display the panes

    let split_pane = gtk::Box::new(Orientation::Horizontal, 10);

    split_pane.set_size_request(-1, -1);
    split_pane.add(&left_tree);
    split_pane.add(&right_tree);

    window.add(&split_pane);
    window.show_all();
    gtk::main();
}
