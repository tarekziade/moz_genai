# https://huggingface.co/dslim/bert-base-NER
import requests
from collections import defaultdict
from bs4 import BeautifulSoup
from transformers import AutoTokenizer, AutoModelForTokenClassification
from transformers import pipeline

tokenizer = AutoTokenizer.from_pretrained("dslim/bert-base-NER")
model = AutoModelForTokenClassification.from_pretrained("dslim/bert-base-NER")

nlp = pipeline("ner", model=model, tokenizer=tokenizer)


defs = {
    "O": "?",  # Outside of a named entity
    "B-MISC": "Misc",  # 	Beginning of a miscellaneous entity right after another miscellaneous entity
    "I-MISC": "Misc",  # 	Miscellaneous entity
    "B-PER": "Person",  # 	Beginning of a person’s name right after another person’s name
    "I-PER": "Person",  # 	Person’s name
    "B-ORG": "Organization",  # 	Beginning of an organization right after another organization
    "I-ORG": "Organization",
    "B-LOC": "Location",  # 	Beginning of a location right after another location
    "I-LOC": "Location",
}


def extract_text_from_url(url):
    print(f"Working on {url}")
    try:
        response = requests.get(url)
        if response.status_code == 200:
            soup = BeautifulSoup(response.content, "html.parser")
            text = " ".join([p.get_text() for p in soup.find_all("p")])
            text = text.replace("\n", " ").strip()
            print(f"Text length: {len(text)}")
            return text
        else:
            print(f"Failed to fetch content. Status code: {response.status_code}")
            return None
    except requests.RequestException as e:
        print(f"Request Exception: {e}")
        return None


with open("pages.txt") as f:
    urls = f.read().splitlines()


def spacer(text):
    new_text = ""
    start = 0
    length = len(text)

    while start < length:
        end = start + 1

        while end < length and text[start : end + 1].isupper():
            end += 1

        new_text += text[start:end]

        if end < length and text[end].isupper():
            new_text += " "

        start = end

    return new_text


for url in urls:
    # Extract text content from the URL and clean HTML
    web_content = extract_text_from_url(url)

    words = []

    for ner in nlp(web_content):
        kind = defs[ner["entity"]]
        word = ner["word"].replace("#", "").strip()

        if ner["entity"][0] == "B":
            words.append((word, kind))
        elif ner["entity"][0] == "I":
            p_word, kind = words[-1]
            p_word += word
            words[-1] = p_word, kind

    by_kind = defaultdict(set)
    for word, kind in words:
        word = spacer(word)
        if len(word) < 3:
            continue
        by_kind[kind].add(word)

    for kind, words in by_kind.items():
        print(f"{kind} => {', '.join(words)}")
    print()
