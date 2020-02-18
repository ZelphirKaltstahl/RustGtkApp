extern crate pango;
extern crate gtk;

use self::pango::{Alignment};
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
    // pub attribute_label: Label,
    pub attribute_combobox: ComboBox,
    pub character_textview: TextView,
    pub character_textbuffer: TextBuffer,
    pub left_right_button_box: ButtonBox,
    pub left_button: Button,
    pub right_button: Button
}

impl VocBigCharacterBox {
    pub fn new() -> VocBigCharacterBox {
        // the box containing it all
        let container: Box = Box::new(Orientation::Vertical, 0);

        // the heading for the big character box telling the user what it shows
        let heading_label_alignment: Alignment = Alignment::new(0.5, 0.5, 1.0, 1.0);  //xalign: f32, yalign: f32, xscale: f32, yscale: f32
        let heading_label: Label = Label::new(Some("Characters"));
        //heading_label_alignment.

        // let attribute_label: Label = Label::new("Atrribute");
        let attribute_combobox: ComboBox = ComboBox::new();
        attribute_combobox.set_title("Attr:");

        // a TextView for the big character to be displayed in
        let character_textview: TextView = TextView::new();
        let character_textbuffer: TextBuffer = character_textview.get_buffer()
            .expect("initialized TextView does not have a TextBuffer");
        //let big_character_tag: TextTag = character_textbuffer.create_


        let left_right_button_box: ButtonBox = ButtonBox::new(Orientation::Horizontal);
        let left_button: Button = Button::new_with_label("next");
        let right_button: Button = Button::new_with_label("prev");

        // adding it all
        container.pack_start(&heading_label, false, false, 0);
        container.pack_start(&attribute_combobox, false, false, 0);

        VocBigCharacterBox {
            container: container,
            heading_label: heading_label,
            // attribute_label: attribute_label,
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
