extern crate gtk;

use self::gtk::prelude::*;
use self::gtk::{
    CellRendererText,
    CellRendererToggle,
    ListStore,
    TreeModel,
    TreeStore,
    TreeView,
    TreeViewColumn,
};

// use vocabulary::Vocabulary;

pub struct VocTreeView {
    pub tree_view: TreeView,
    pub model: TreeStore,
}

impl VocTreeView {
    pub fn new() -> VocTreeView {
        let column_types = [String::static_type()];
        let model = TreeStore::new(&column_types);
        let tree_view = TreeView::new_with_model(&model);

        let renderer = CellRendererText::new();
        let column = TreeViewColumn::new();
        column.set_title("Column Title");
        column.pack_start(&renderer, false);
        column.add_attribute(&renderer, "text", 0);
        tree_view.append_column(&column);

        VocTreeView {
            tree_view: tree_view,
            model: model
        }
    }

    // pub fn create_with_vocabulary(&mut self, vocabulary: Vocabulary) -> VocTreeView {
    //     // TODO
    // }

    fn append_text_column(&mut self, column_name: String) {
        let column = TreeViewColumn::new();
        let cell_renderer = CellRendererText::new();

        column.set_title(&column_name);
        column.pack_start(&cell_renderer, true);
        column.add_attribute(&cell_renderer, "text", 0);
        self.tree_view.append_column(&column);
    }

    fn append_toggle_column(&mut self, column_name: String) {
        let column = TreeViewColumn::new();
        let cell_renderer = CellRendererToggle::new();

        column.pack_start(&cell_renderer, true);
        column.add_attribute(&cell_renderer, "text", 0);
        self.tree_view.append_column(&column);
    }
}
