import os
import requests
from transformers import (
    AutoTokenizer,
    AutoModelForSeq2SeqLM,
    AutoModelForCausalLM,
)
from bs4 import BeautifulSoup
from transformers import MistralModel, MistralForCausalLM
import sys
import time
import json
import nltk


os.environ["TOKENIZERS_PARALLELISM"] = "false"


MODELS = (
    ("openai", None, None),
    (
        "t5-small-xsum",
        AutoModelForSeq2SeqLM.from_pretrained("t5-small", return_dict=True),
        AutoTokenizer.from_pretrained("t5-small"),
    ),
    (
        "t5-small",
        AutoModelForSeq2SeqLM.from_pretrained("t5-small", return_dict=True),
        AutoTokenizer.from_pretrained("t5-small"),
    ),
    (
        "t5-base",
        AutoModelForSeq2SeqLM.from_pretrained("t5-base", return_dict=True),
        AutoTokenizer.from_pretrained("t5-base"),
    ),
    # (
    #    "t5-large",
    #    AutoModelForSeq2SeqLM.from_pretrained("t5-large", return_dict=True),
    #    AutoTokenizer.from_pretrained("t5-large", model_max_length=10000),
    # ),
    # (
    #    "Intel/neural-chat-7b-v3-1",
    #    MistralForCausalLM.from_pretrained(
    #        "Intel/neural-chat-7b-v3-1", return_dict=True
    #    ),
    #    AutoTokenizer.from_pretrained("Intel/neural-chat-7b-v3-1"),
    # ),
)

print("Models loaded")
for name, model, _ in MODELS:
    if name == "openai":
        continue
    model_size_bytes = sum(p.numel() for p in model.parameters() if p.requires_grad) * 4
    model_size_mib = model_size_bytes / (1024 * 1024)
    print(f"{name} => size: {model_size_mib:.2f} MiB")
print("")


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


def generate_summary(text, model, tokenizer):
    if name == "openai":
        from openai import OpenAI

        client = OpenAI()

        response = client.chat.completions.create(
            model="gpt-3.5-turbo-1106",
            messages=[
                {
                    "role": "system",
                    "content": "You will be provided with a text, return a summary which length is at most one sentence as short as possible",
                },
                {"role": "user", "content": text},
            ],
        )

        return response.choices[0].message.content

    inputs = tokenizer.encode(
        "summarize: " + text, return_tensors="pt", truncation=True, max_length=2048
    )

    output = model.generate(
        inputs, max_length=50, num_beams=4, length_penalty=2.0, early_stopping=True
    )
    summary = tokenizer.decode(output[0], skip_special_tokens=True)
    sentences = nltk.sent_tokenize(summary)
    return sentences[0].capitalize()


with open("pages.txt") as f:
    urls = f.read().splitlines()

for url in urls:
    # Extract text content from the URL and clean HTML
    web_content = extract_text_from_url(url)

    if web_content:
        for name, model, tokenizer in MODELS:
            start = time.time()
            summarized_text = generate_summary(web_content, model, tokenizer)
            print(f'- {name} : "{summarized_text}" - took {time.time()-start:.2f}s\n')
    else:
        print("Failed to fetch content or invalid URL.")
