//#[macro_use] extern crate serde_derive;
extern crate serde;

extern crate gtk;
use self::gtk::prelude::*;
//use self::gtk::Type::*;
use self::gtk::*;
/*{
    Alignment,
    Box,
    ButtonBox,
    Label,
    Menu,
    MenuBar,
    MenuItem,
    Notebook,
    Orientation,
    TreeView,
    Window,
    WindowExt,
    WindowType,
};*/

mod voc_notebook;
use voc_notebook::{VocNotebook};

mod csv_reading_and_writing;

mod vocabulary;
use vocabulary::{Vocabulary, Metadata, Word, WordMetadata, WordTranslation};

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

fn main() {
    run_gtk_example();
}

pub fn run_gtk_example() {
    initialize_gtk();
    let window : Window = Window::new(WindowType::Toplevel);
    configure_window(&window);

    let vbox = Box::new(Orientation::Vertical, 0);
    let menubar = create_menu_bar();
    let mut notebook = create_notebook();

    vbox.pack_start(&menubar, false, false, 0); // child expand fill padding
    vbox.pack_start(&notebook.notebook, true, true, 0);

    notebook.create_tab_from_str("dynamically added");

    window.add(&vbox);
    window.show_all();
    // Handle closing of the window.
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    // Run the main loop.
    gtk::main();
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

pub fn create_notebook() -> VocNotebook {
    let mut notebook = VocNotebook::new();
    let tab_titles : Vec<&str> = vec!(
        "Library", "Training", "Statistics", "not needed");

    for tab_title in tab_titles {
        let label = gtk::Label::new(tab_title);
        let index : u32 = notebook.create_tab(tab_title, label.upcast());
        println!("adding a tab with index {}", index);
    }

    let tab_content = notebook.notebook.get_nth_page(Some(3)).expect("could not find such notebook page");

    println!("{:?}", tab_content);
    notebook
}

pub fn initialize_gtk() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
}
