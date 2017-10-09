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

use vocabulary::{Vocabulary, Word};

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
        let lang_ids: Vec<String> = VocTreeView::get_lang_ids(&vocabulary);
        // let lang_names: Vec<String> = VocTreeView::get_lang_names(&vocabulary);
        let number_of_columns: usize = VocTreeView::get_number_of_columns(&vocabulary);
        let mut column_types: Vec<gtk::Type> = Vec::new();
        let mut column_indices: Vec<u32> = Vec::new();

        for index in 0..number_of_columns {
            column_types.push(String::static_type());
            column_indices.push(index as u32);
        }

        let model = TreeStore::new(&column_types);
        let mut tree_view = TreeView::new_with_model(&model);

        tree_view.get_selection().set_mode(gtk::SelectionMode::Multiple);

        for index in 0..number_of_columns {
            VocTreeView::append_text_column(
                &mut tree_view,
                vocabulary.metadata.language_id_to_name.get(&lang_ids[index]).unwrap().clone(),
                index as i32)
        }

        VocTreeView::add_words_to_model(&model, vocabulary, lang_ids, number_of_columns, column_indices);
        VocTreeView {
            tree_view: tree_view,
            model: model
        }
    }

    pub fn add_words_to_model(
        model: &TreeStore,
        vocabulary: Vocabulary,
        lang_ids: Vec<String>,
        number_of_columns: usize,
        column_indices: Vec<u32>
    ) {
        let words = vocabulary.words;
        for word in words {
            for meaning in word.meanings {
                let mut items: Vec<String> = Vec::new();
                for index in 0..number_of_columns {
                    let item_to_add = meaning.translation.get(&lang_ids[index]).expect(
                        &format!("{} {}",
                                 "Not as many translation attributes as lang_ids in the metadata of the JSON.",
                                 "Are you sure the JSON is valid?"))
                        .clone();
                    items.push(item_to_add);
                }

                assert!(items.len() == lang_ids.len(),
                        "items length was {} with items being {:?}",
                        items.len(), items);
                VocTreeView::add_to_tree_store(&model, &column_indices, items);
            }
        }
    }

    pub fn get_number_of_columns(vocabulary: &Vocabulary) -> usize {
        let lang_ids: Vec<String> = VocTreeView::get_lang_ids(&vocabulary);
        let lang_names: Vec<String> = VocTreeView::get_lang_names(&vocabulary);

        assert!(lang_ids.len() == lang_names.len(),
                format!("number of language ids is not equal to number of language names.{}",
                        format!("lang_ids has length: {} lang_names has length: {}",
                                lang_ids.len(),
                                lang_names.len())));

        let number_of_columns = lang_ids.iter().len();
        number_of_columns
    }

    pub fn get_lang_ids(vocabulary: &Vocabulary) -> Vec<String> {
        vocabulary.metadata.language_id_to_name.iter().map(
            |(key, _value)| key.clone()
        ).collect()
    }

    pub fn get_lang_names(vocabulary: &Vocabulary) -> Vec<String> {
        vocabulary.metadata.language_id_to_name.iter().map(
            |(_key, value)| value.clone()
        ).collect()
    }

    pub fn add_to_tree_store(tree_store: &TreeStore, column_indices: &[u32], row: Vec<String>) {
        println!("COLUMNS ARE: {:?}", column_indices);
        assert!(column_indices.len() == row.len(),
                "column indices length != row length: {} to {}",
                column_indices.len(), row.len());

        let inserted_row = VocTreeView::string_to_ToValue(&row);

        assert!(column_indices.len() == row.len(),
                "column indices length != row length: {} to {}",
                column_indices.len(), inserted_row.len());

        println!("COLUMNS ARE: {:?}", column_indices);
        println!("ROW IS: {:?}", row);
        tree_store.insert_with_values(None, None, column_indices, &inserted_row);
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

    pub fn append_toggle_column(tree_view: &mut TreeView, column_name: String, column_index: i32) {
        let column = TreeViewColumn::new();
        let cell_renderer = CellRendererToggle::new();

        column.set_sizing(gtk::TreeViewColumnSizing::Fixed);
        column.set_expand(true);
        column.set_reorderable(true);
        column.set_resizable(true);
        column.set_title(&column_name);

        column.pack_start(&cell_renderer, true);
        column.add_attribute(&cell_renderer, "text", column_index);

        tree_view.append_column(&column);
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
