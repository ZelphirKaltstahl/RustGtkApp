extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate pango;

use std::fs::File;
use std::io::prelude::*;

extern crate gtk;
use self::gtk::prelude::*;
// use self::pango::*;
use self::gtk::*;

mod gtk_utils;
use gtk_utils::{make_scrollable, initialize_gtk};

mod include;
use include::voc_notebook::{VocNotebook};
use include::voc_treeview::{VocTreeView};
use include::voc_statusbar::{VocStatusBar};
use include::vocabulary::{Vocabulary};
use include::voc_status_message::{VocStatusMessage};
use include::voc_big_character_box::{VocBigCharacterBox};

mod csv_reading_and_writing;


// macro_rules! hashmap {
//     ($( $key: expr => $val: expr ),*) => {{
//          let mut map = ::std::collections::HashMap::new();
//          $( map.insert($key, $val); )*
//          map
//     }}
// }


fn main() {
    let vocabulary_data: Vec<Vocabulary> = read_data_files();
    for voc in vocabulary_data {
        run_gtk_example(voc);
    }
    // run_gtk_example();
}

pub fn read_data_files() -> Vec<Vocabulary> {
    let mut data: Vec<Vocabulary> = Vec::new();
    let filepaths: Vec<&str> = vec!("data/hsk-1-updated.json");

    for filepath in filepaths {
        let mut f = File::open(filepath).expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents).expect("something went wrong reading the file");

        let serde_value: Vocabulary = serde_json::from_str(&contents).unwrap();
        data.push(serde_value);
    }
    data
}

pub fn run_gtk_example(vocabulary: Vocabulary) {
    initialize_gtk();
    let window: Window = construct_gui(vocabulary);
    window.show_all();
    // Handle closing of the window.
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    // Run the main loop.
    gtk::main();
}

pub fn construct_gui(vocabulary: Vocabulary) -> Window {
    let window : Window = Window::new(WindowType::Toplevel);
    configure_window(&window);

    let vbox = Box::new(Orientation::Vertical, 0);
    let menubar = create_menu_bar();
    let mut notebook = create_notebook(vocabulary);
    let mut status_bar = VocStatusBar::new();
    status_bar.set_message(
        VocStatusMessage {
            message: "Vocabulary loaded.".to_string(),
            context: "initialization".to_string()
        }
    );

    vbox.pack_start(&menubar, false, false, 0); // child expand fill padding
    vbox.pack_start(&notebook.notebook, true, true, 0);
    vbox.pack_start(&status_bar.status_bar, false, false, 0);

    notebook.create_tab_from_str("dynamically added");

    window.add(&vbox);
    window
}

pub fn configure_window(window: &Window) {
    const TITLE: &str = "Vocabulary Trainer";
    const DEFAULT_WIDTH: i32 = 400;
    const DEFAULT_HEIGHT: i32 = 300;
    window.set_title(TITLE);
    window.set_position(WindowPosition::Center);
    window.set_default_size(DEFAULT_WIDTH, DEFAULT_HEIGHT);
}

pub fn create_menu_bar() -> MenuBar {
    // we want to create a menu bar ...
    let menu_bar = MenuBar::new();
    let file_menu : MenuItem = create_file_menu_item();
    // append the File menu item to the MenuBar
    menu_bar.append(&file_menu);
    menu_bar
}

pub fn create_file_menu_item() -> MenuItem {
    // in the menu bar we want a menu item which is labeled "File".
    let file_menu = MenuItem::new_with_label("File");
    // in order to contain menu items itself, the File menu item must have a sub menu.
    // to this sub menu the menu items in the File menu item are appended.
    let file_sub_menu = Menu::new();
    // create menu items
    let new_item = MenuItem::new_with_label("New");
    let exit_item = MenuItem::new_with_label("Exit");
    // append menu items for File menu
    file_sub_menu.append(&new_item);
    file_sub_menu.append(&exit_item);
    // set the submenu when all its children are known
    file_menu.set_submenu(&file_sub_menu);

    exit_item.connect_activate(|_| {
        gtk::main_quit();
    });

    file_menu
}

pub fn create_notebook(vocabulary: Vocabulary) -> VocNotebook {
    let mut notebook = VocNotebook::new();

    // add library tree view
    let library_tab_index : u32 = notebook.create_tab(
        "Library",
        create_library_tab_content(vocabulary));
    let training_tab_index : u32 = notebook.create_tab(
        "Training",
        // TODO: Error here
        gtk::Label::new("Training").upcast());
    let statistics_tab_index : u32 = notebook.create_tab(
        "Training",
        // TODO: Error here
        gtk::Label::new("Training").upcast());

    notebook
}

pub fn create_library_tab_content(vocabulary: Vocabulary) -> Widget {
    let hbox = Box::new(Orientation::Horizontal, 0);
    let voc_tree_view: VocTreeView = create_library_treeview(vocabulary);
    let scrollable_voc_tree_view = make_scrollable(&voc_tree_view.tree_view.upcast());
    let voc_big_character_box = VocBigCharacterBox::new();
    hbox.pack_start(&scrollable_voc_tree_view, true, true, 0);
    hbox.pack_start(&voc_big_character_box.container, false, false, 0);
    hbox.upcast()
}

pub fn create_library_treeview(vocabulary: Vocabulary) -> VocTreeView {
    let voc_tree_view: VocTreeView = VocTreeView::new_with_vocabulary(vocabulary);
    voc_tree_view.tree_view.set_headers_visible(true);
    voc_tree_view
}
