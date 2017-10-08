extern crate pango;
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

use vocabulary::{Vocabulary};

// use vocabulary::Vocabulary;

pub struct VocTreeView {
    pub tree_view: TreeView,
    pub model: TreeStore,
}

// use std::fmt;
// impl fmt::Debug for gtk::ToValue {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Hi: {}", self.id)
//     }
// }

impl VocTreeView {
    pub fn new() -> VocTreeView {
        let column_types = [String::static_type()];
        let model = TreeStore::new(&column_types);
        let tree_view = TreeView::new_with_model(&model);

        // let renderer = CellRendererText::new();
        // let column = TreeViewColumn::new();
        // column.set_title("Column Title");
        // column.pack_start(&renderer, false);
        // column.add_attribute(&renderer, "text", 0);
        // tree_view.append_column(&column);

        VocTreeView {
            tree_view: tree_view,
            model: model
        }
    }

    pub fn new_with_vocabulary(vocabulary: Vocabulary) -> VocTreeView {
        let column_types = [String::static_type(),
                            String::static_type(),
                            String::static_type(),
                            String::static_type()];
        let model = TreeStore::new(&column_types);
        let mut tree_view = TreeView::new_with_model(&model);

        for lang_name in vec!(vocabulary.metadata.first_language_name,
                              vocabulary.metadata.first_language_phonetic_script_name,
                              vocabulary.metadata.second_language_name,
                              vocabulary.metadata.second_language_phonetic_script_name) {
            VocTreeView::append_text_column(&mut tree_view, lang_name);
        }

        for word in vocabulary.words {
            for meaning in word.meanings {
                let mut items: Vec<String> = Vec::new();
                items.push(
                    meaning.translation.get("english").unwrap().to_string());
                items.push(
                    meaning.translation.get("english_phonetic_script").unwrap().to_string());
                items.push(
                    meaning.translation.get("pinyin").unwrap().to_string());
                items.push(
                    meaning.translation.get("chinese_simplified").unwrap().to_string());
                // for (_attr, item) in meaning.translation {
                //     items.push(item);
                // }
                // println!("{:?}", items);
                assert!(items.len() == 4,
                        "items length was {} with items being {:?}", items.len(), items);
                let columns: &[u32] = &[0,1,2,3];
                VocTreeView::add_to_tree_store(&model, columns, items);
            }
        }

        VocTreeView {
            tree_view: tree_view,
            model: model
        }
    }

    pub fn add_to_tree_store(tree_store: &TreeStore, columns: &[u32], row: Vec<String>) {
        tree_store.insert_with_values(
            None,
            None,
            columns,
            &VocTreeView::string_to_ToValue(&row)
        );
    }

    pub fn string_to_ToValue(values_vector: &[String]) -> Vec<&gtk::ToValue> {
        values_vector.into_iter().map(
            |item| item as &gtk::ToValue
        ).collect()
    }

    // pub fn strings_to_ampersand_str<'res_ref>(values_vector: Vec<String>) -> Vec<&'res_ref str> {
    //     let append_values: Vec<_> = values_vector.iter().map(|x| &x[..]).collect();
    //     append_values
    // }

    pub fn append_text_column(tree_view: &mut TreeView, column_name: String) {
        let column = TreeViewColumn::new();
        let cell_renderer = CellRendererText::new();
        cell_renderer.set_property_ellipsize(pango::EllipsizeMode::End);

        column.set_sizing(gtk::TreeViewColumnSizing::Fixed);
        column.set_expand(true);
        column.set_reorderable(true);
        column.set_resizable(true);
        column.set_title(&column_name);

        column.pack_start(&cell_renderer, true);
        column.add_attribute(&cell_renderer, "text", 0);

        tree_view.append_column(&column);
    }

    pub fn append_toggle_column(&mut self, column_name: String) {
        let column = TreeViewColumn::new();
        let cell_renderer = CellRendererToggle::new();

        column.pack_start(&cell_renderer, true);
        column.add_attribute(&cell_renderer, "text", 0);
        self.tree_view.append_column(&column);
    }
}
