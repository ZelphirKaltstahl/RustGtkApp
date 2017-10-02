fn create_vocabulary_test() -> Vocabulary {
    let mut a_vocabulary : Vocabulary = Vocabulary {
        metadata: Metadata {
            identifier: String::from("HSK1"),
            source_note: String::from("vocabulary taken from http://data.hskhsk.com/lists/ created by Alan Davies, alan@hskhsk.com 2013-2017")
        },
        words: vec![
            Word {
                metadata: WordMetadata {
                    learned: false,
                    relevance_level: 5,
                    tags: vec!(String::from("verb"))
                },
                translations: vec!(WordTranslation {
                    translations: hashmap!{
                        String::from("english") => String::from("love"),
                        String::from("pinyin_numbered") => String::from("ai4"),
                        String::from("pinyin") => String::from("ài"),
                        String::from("chinese_simplified") => String::from("爱"),
                        String::from("chinese_traditional") => String::from("愛")
                    },
                    description: vec![String::from("This is a very clichee word, which everyone learns early on.")],
                    examples: vec![String::from("我爱你.")]
                })
            },
            Word {
                metadata: WordMetadata {
                    learned: false,
                    relevance_level: 5,
                    tags: vec![String::from("verb")]
                },
                translations: vec!(WordTranslation {
                    translations: hashmap!{
                        String::from("english") => String::from("love"),
                        String::from("pinyin_numbered") => String::from("ai4"),
                        String::from("pinyin") => String::from("ài"),
                        String::from("chinese_simplified") => String::from("爱"),
                        String::from("chinese_traditional") => String::from("愛")
                    },
                    description: vec!(String::from("This is a very clichee word, which everyone learns early on.")),
                    examples: vec!(String::from("我爱你."))
                })
            },
        ]
    };

    a_vocabulary.words[0].metadata.learned = true;
    println!("{:?}", a_vocabulary.words[0].metadata.learned);

    a_vocabulary
}
