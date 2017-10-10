extern crate pango;
extern crate gtk;

use self::gtk::prelude::*;
use self::gtk::{
    TextView,
    TextBuffer,
    TextTagTable
};

pub struct VocTextView {
    pub view: TextView,
    pub model: TextBuffer,
}

impl VocTextView {
    pub fn new() -> VocTextView {
        let view: TextView = TextView::new();
        let model: TextBuffer = view.get_buffer().expect("initialized TextView does not yet have a TextBuffer");

        VocTextView {
            view: view,
            model: model
        }
    }

    pub fn new_with_characters(characters: String) -> VocTextView {
        let view: TextView = TextView::new();
        let model: TextBuffer = view.get_buffer().expect("initialized TextView does not yet have a TextBuffer");
        // TODO
        VocTextView {
            view: view,
            model: model
        }
    }

}
