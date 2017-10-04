extern crate serde;
extern crate serde_json;

extern crate rustc_serialize;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Vocabulary {
    pub metadata: Metadata,
    pub words: Vec<Word>
}

#[derive(Debug)]
pub struct Metadata {
    pub identifier: String,
    pub source_note: String
}

#[derive(Debug)]
pub struct Word {
    pub metadata: WordMetadata,
    pub translations: Vec<WordTranslation>
}

#[derive(Debug)]
pub struct WordMetadata {
    pub identifier: String,
    pub learned: bool,
    pub relevance_level: u8,
    pub tags: Vec<String>
}

#[derive(Debug)]
pub struct WordTranslation {
    pub translations: HashMap<String, String>,  // for example: german:Ich, chinese_simplified:..., chinese_traditional:...
    pub description: Vec<String>,  // a single translation might have multiple ways of explanation
    pub examples: Vec<String>,  // and multiple examples
}

/*
impl fmt::Debug for Vocabulary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vocabulary {{ name: {}, gadgets: {:?} }}")
    }
}
*/
