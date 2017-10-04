//! This is the documentation for `voc_notebook`
//!
//! # Description
//!
//! This module implements convinience procedures for gtk::Notebook usage.
//!
//! # Tests
//!
//! ```
//! assert_eq!(4, 2 + 2);
//! ```
extern crate gtk;

use self::gtk::prelude::*;
use self::gtk::{
    Box, Button, Container, IconSize, Image, Label, Notebook, Orientation, ReliefStyle, Widget};

pub struct VocNotebook {
    pub notebook: Notebook,
    pub tabs: Vec<Box>
}

impl VocNotebook {
    pub fn new() -> VocNotebook {
        VocNotebook {
            notebook: Notebook::new(),
            tabs: Vec::new()
        }
    }

    /// # Description
    /// adds a tab to the notebook
    /// # Arguments
    /// * `title`: the title of the tab
    /// * `widget`: content of the tab
    pub fn create_tab(&mut self, title: &str, widget: Widget) -> u32 {
        let tab = Box::new(Orientation::Horizontal, 0);

        let button = Button::new();

        // we want a label an image for the closing button
        let close_image = Image::new_from_icon_name("window-close", IconSize::Button.into());
        let label = Label::new(title);

        // some button styling
        button.set_relief(ReliefStyle::None);
        button.set_focus_on_click(false);
        // add an image to the button
        button.add(&close_image);

        // add both the label containing text and the tab closing button to the tab
        tab.pack_start(&label, false, false, 0);
        tab.pack_start(&button, false, false, 0);
        // probably needed
        tab.show_all();

        // index of the added page (tab content)
        let index : u32 = self.notebook.append_page(&widget, Some(&tab));

        let notebook_clone = self.notebook.clone();
        button.connect_clicked(
            move |_| {
                let index = notebook_clone
                    .page_num(&widget)
                    .expect("Couldn't get page_num from notebook_clone");
                notebook_clone.remove_page(Some(index));
            }
        );

        self.tabs.push(tab);

        index
    }

    pub fn create_tab_from_str(&mut self, title: &str) -> u32 {
        let label = Label::new(title);
        self.create_tab(&title, label.upcast())
    }
}
