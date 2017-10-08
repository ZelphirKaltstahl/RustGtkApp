extern crate pango;
extern crate gtk;

use std;
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

pub struct VocTreeView {
    pub tree_view: TreeView,
    pub model: TreeStore,
}

impl VocTreeView {
    pub fn new() -> VocTreeView {
        let column_types = [String::static_type()];
        let model = TreeStore::new(&column_types);
        let tree_view = TreeView::new_with_model(&model);

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
        tree_view.get_selection().set_mode(gtk::SelectionMode::Multiple);

        let lang_ids: Vec<String> = vocabulary.metadata.language_id_to_name.iter().map(
            |(key, _value)| key.clone()
        ).collect();

        let lang_names: Vec<String> = vocabulary.metadata.language_id_to_name.iter().map(
            |(_key, value)| value.clone()
        ).collect();

        let mut column_indices: Vec<u32> = Vec::new();
        for (index, _lang_name) in lang_names.into_iter().enumerate() {
            column_indices.push(index as u32);
        }

        // let mut column_indices = &[0; lang_ids.len()];
        // for (index, lang_name) in lang_names.iter().enumerate() {
        //     column_indices.push(index as u32);
        // }

        assert!(lang_ids.len() == lang_names.len(),
                format!("number of language ids is not equal to number of language names.{}",
                        format!("lang_ids has length: {} lang_names has length: {}",
                                lang_ids.len(),
                                lang_names.len())));

        for index in 0..lang_ids.len() {
            VocTreeView::append_text_column(
                &mut tree_view,
                vocabulary.metadata.language_id_to_name.get(&lang_ids[index]).unwrap().clone(),
                index as i32)
        }

        for word in vocabulary.words {
            for meaning in word.meanings {
                let mut items: Vec<String> = Vec::new();
                for index in 0..lang_ids.len() {
                    items.push(meaning.translation.get(&lang_ids[index])
                               .expect("Not as many translation attributes as lang_ids defined in the metadata of the JSON. Are you sure the JSON is valid?")
                               .clone());
                }

                assert!(items.len() == lang_ids.len(),
                        "items length was {} with items being {:?}", items.len(), items);
                let columns: &[u32] = &[];
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
        let res = values_vector.into_iter().map(
            |item| item as &gtk::ToValue
        ).collect();
        // println!("{:?}", DebugPrintableToValue(res));  // this print seems impossible
        res
    }

    pub fn append_text_column(tree_view: &mut TreeView, column_name: String, column_index: i32) {
        let column = TreeViewColumn::new();
        let cell_renderer = CellRendererText::new();
        cell_renderer.set_property_ellipsize(pango::EllipsizeMode::End);

        column.set_sizing(gtk::TreeViewColumnSizing::Fixed);
        column.set_expand(true);
        column.set_reorderable(true);
        column.set_resizable(true);
        column.set_title(&column_name);

        column.pack_start(&cell_renderer, true);
        column.add_attribute(&cell_renderer, "text", column_index);

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

// use std::fmt;
// use std::fmt::Debug;
// pub struct DebugPrintableToValue(pub gtk::ToValue);

// impl Debug for DebugPrintableToValue {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "gtk::ToValue id: {}", self)
//     }
// }
