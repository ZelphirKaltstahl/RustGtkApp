extern crate pango;
extern crate gtk;

use self::gtk::prelude::*;
use self::gtk::{
    Statusbar
};

use include::voc_status_message::VocStatusMessage;
//use voc_status_message::{VocStatusMessage};

pub struct VocStatusBar {
    pub status_bar: Statusbar
}

impl VocStatusBar {
    pub fn new() -> VocStatusBar {
        let bar: Statusbar = Statusbar::new();
        // TODO
        VocStatusBar {
            status_bar: bar
        }
    }

    pub fn set_message(&mut self, status_message: VocStatusMessage) {
        let context_id: u32 = self.status_bar.get_context_id(&status_message.context);

        // remove all previous messages with the same context description
        self.status_bar.remove_all(context_id);

        let message_id: u32 = self.status_bar.push(context_id, &status_message.message);
    }
}
