extern crate serde;
extern crate serde_json;

extern crate rustc_serialize;

use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct Vocabulary {
    pub metadata: Metadata,
    pub words: Vec<Word>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Metadata {
    pub identifier: String,
    pub source_note: String,
    pub first_language_name: String,
    pub first_language_phonetic_script_name: String,
    pub second_language_name: String,
    pub second_language_phonetic_script_name: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Word {
    pub metadata: WordMetadata,
    pub meanings: Vec<WordMeaning>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WordMetadata {
    pub identifier: String,
    pub learned: bool,
    pub relevance_level: u8,
    pub tags: Vec<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WordMeaning {
    pub translation: HashMap<String, String>,
    pub description: Vec<String>,
    pub examples: Vec<String>
}

/*
impl fmt::Debug for Vocabulary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vocabulary {{ name: {}, gadgets: {:?} }}")
    }
}
*/
