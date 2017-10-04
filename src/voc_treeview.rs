extern crate gtk;

use self::gtk::prelude::*;
use self::gtk::{TreeView, TreeModel, ListStore, TreeStore};

pub struct VocTreeView {
    pub tree_view: TreeView,
    pub model: TreeModel,
}

impl VocTreeView {
    pub fn new() -> VocTreeView {
        let model = ListStore::new(&[U64, String, String, String, String, Bool]);
        let tree_view = TreeView::new_with_model(model);

        VocTreeView {
            tree_view: tree_view,
            model: model
        }
    }

    pub fn create_with_vocabulary(&mut self, vocabulary: Vocabulary) -> VocTreeView {
        // TODO
    }
}
