import json


def read(input_file_name):
    with open(input_file_name, mode="r") as opened_file:
        content = json.load(opened_file)
    return content


def convert(json_content):
    vocabulary_metadata = json_content["metadata"]
    del vocabulary_metadata["learned_percentage"]
    del vocabulary_metadata["count"]
    vocabulary_metadata["language_id_to_name"] = {
        "english": "English",
        "english_phonetic_script": "IPA",
        "chinese_simplified": "Chinese (simplified)",
        "chinese_traditional": "Chinese (traditional)",
        "pinyin": "Pīnyīn",
        "pinyin_numbered": "Pīnyīn numbered"
    }

    words = []
    for ind in json_content["words"]:
        word = json_content["words"][ind]
        print(f"parsing word: {word}")
        words.append({
            "metadata": {
                "identifier": ind,
                "learned": word["metadata"]["learned"],
                "relevance_level": 5,
                "tags": []
            },
            "meanings": [
                {
                    "translation": {
                        "english": word["translation_data"]["english"],
                        "english_phonetic_script": "(add IPA)",
                        "pinyin": word["translation_data"]["pinyin"],
                        "pinyin_numbered": word["translation_data"]["pinyin_numbered"],
                        "chinese_simplified": word["translation_data"]["simplified"],
                        "chinese_traditional": word["translation_data"]["traditional"]
                    },
                    "description": [],
                    "examples": []
                }
            ]
        })
        pass

    updated_vocabulary = {
        "metadata": vocabulary_metadata,
        "words": words
    }
    return updated_vocabulary


def write(content, output_file_name):
    with open(output_file_name, mode='w', encoding='utf8') as write_file:
        write_file.write(json.dumps(content, indent=4, ensure_ascii=False))


if __name__ == "__main__":
    for ind in range(1,7):
        write(convert(read(f"hsk-{ind}.json")), f"hsk-{ind}-updated.json")
