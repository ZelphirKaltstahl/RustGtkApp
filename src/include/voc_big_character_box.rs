extern crate gtk;

use self::gtk::prelude::*;
use self::gtk::{
    Box,
    Button,
    ButtonBox,
    ComboBox,
    Label,
    Orientation,
    TextBuffer,
    TextTag,
    TextTagTable,
    TextView
};


pub struct VocBigCharacterBox {
    pub container: Box,
    pub heading_label: Label,
    pub attribute_label: Label,
    pub attribute_combobox: ComboBox,
    pub character_textview: TextView,
    pub character_textbuffer: TextBuffer,
    pub left_right_button_box: ButtonBox,
    pub left_button: Button,
    pub right_button: Button
}

impl VocBigCharacterBox {
    pub fn new() -> VocBigCharacterBox {
        let container: Box = Box::new(Orientation::Horizontal, 0);
        let heading_label: Label = Label::new("Characters");
        let attribute_label: Label = Label::new("Atrribute");
        let attribute_combobox: ComboBox = ComboBox::new();
        let character_textview: TextView = TextView::new();
        let character_textbuffer: TextBuffer = character_textview.get_buffer()
            .expect("initialized TextView does not have a TextBuffer");
        let left_right_button_box: ButtonBox = ButtonBox::new(Orientation::Horizontal);
        let left_button: Button = Button::new_with_label("next");
        let right_button: Button = Button::new_with_label("prev");

        container.pack_start(&heading_label, false, false, 0);

        VocBigCharacterBox {
            container: container,
            heading_label: heading_label,
            attribute_label: attribute_label,
            attribute_combobox: attribute_combobox,
            character_textview: character_textview,
            character_textbuffer: character_textbuffer,
            left_right_button_box: left_right_button_box,
            left_button: left_button,
            right_button: right_button
        }
    }

    pub fn show_all(&mut self) {
        self.container.show_all();
    }
}
