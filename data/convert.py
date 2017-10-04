import json


def read(input_file_name):
    with open(input_file_name, mode="r") as opened_file:
        content = json.load(opened_file)
    return content


def convert(json_content):
    vocabulary_metadata = json_content["metadata"]
    del vocabulary_metadata["learned_percentage"]
    del vocabulary_metadata["count"]

    words = []
    for ind, word in enumerate(json_content["words"]):
        print(f"parsing word: {ind}")
        # words.append()

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
