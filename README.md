# Experiment

Extreme summarization of web pages using OpenAI and various local models.

Results on an Apple M1:

```
Models loaded
Falconsai/text_summarization => size: 230.81 MiB
adasnew/t5-small-xsum => size: 230.81 MiB
t5-small => size: 230.81 MiB
t5-base => size: 850.31 MiB

Working on https://en.wikipedia.org/wiki/Sandman
Text length: 9577
- openai : "The Sandman is a mythical character from European folklore who puts people to sleep and inspires beautiful dreams by sprinkling magical sand onto their eyes, also featured in children's stories, music, and various forms of media." - took 1.79s

- Falconsai/text_summarization : "The sandman is a mythical character in many children's stories and books." - took 2.40s

- adasnew/t5-small-xsum : "The sandman is a fictional character in a series of comic books based on the story of a young boy who sprinkled sand on his eyes." - took 1.90s

- t5-small : "The sandman is a mythical character in many children's stories and books." - took 2.33s

- t5-base : "The sandman is a mythical character in many children's stories and books." - took 7.50s

Working on https://en.wikipedia.org/wiki/Named-entity_recognition
Text length: 9705
- openai : "Named-entity recognition (NER) is a subtask of information extraction that locates and classifies named entities mentioned in unstructured text into pre-defined categories such as person names, organizations, locations, medical codes, and time expressions." - took 2.09s

- Falconsai/text_summarization : "Named-entity recognition (ner) is a subtask of information extraction that seeks to locate and classify named entities mentioned in unstructured text into pre-defined categories such as person names, organizations, locations, medical codes" - took 2.60s

- adasnew/t5-small-xsum : "A number of ner systems have been developed to help identify and classify names of people in the context of an unstructured text." - took 1.78s

- t5-small : "Ner is a subtask of information extraction that seeks to locate and classify named entities." - took 2.50s

- t5-base : "Named-entity recognition (ner) is a subtask of information extraction." - took 7.17s

Working on https://en.wikipedia.org/wiki/Firefox
Text length: 50846
- openai : "Mozilla Firefox is a free and open-source web browser developed by the Mozilla Foundation and is available for various operating systems, including Windows 10, macOS, Linux, Android, and iOS, with the latest usage share of 5.96% as a desktop web browser and 3.04% across all platforms as of September 2023." - took 3.76s

- Falconsai/text_summarization : "Mozilla firefox, or simply firefox, is a free and open-source[11] web browser developed by the mozilla foundation and its subsidiary, the mozilla corporation." - took 2.56s

- adasnew/t5-small-xsum : "Mozilla firefox has been named the "biggest update" to its web browser since the release of 1.0." - took 1.54s

- t5-small : "Mozilla firefox is a free and open-source web browser developed by the mozilla foundation and its subsidiary, the mozilla corporation." - took 2.68s

- t5-base : "Mozilla firefox, or simply firefox, is a free and open-source[11] web browser." - took 7.19s

Working on https://en.wikipedia.org/wiki/Mozilla
Text length: 27948
- openai : "Mozilla is a free software community founded in 1998 and best known for creating the Firefox web browser and Thunderbird email client, it is also involved in various other projects and initiatives such as the Mozilla Manifesto, the creation of WebXR Viewer, and the launch of the Common Voice project." - took 2.19s

- Falconsai/text_summarization : "Mozilla is a free software community founded in 1998 by members of netscape." - took 2.78s

- adasnew/t5-small-xsum : "Mozilla has announced that it will launch a premium version of the mozilla web browser in 2019." - took 1.32s

- t5-small : "The mozilla community uses, develops, publishes and supports mozilla products." - took 2.65s

- t5-base : "Mozilla is a free software community founded in 1998 by members of netscape." - took 6.98s

Working on https://en.wikipedia.org/wiki/Moore%27s_law
Text length: 27590
- openai : "Moore's Law is an observation that the number of transistors in an integrated circuit doubles approximately every two years, impacting advancements in digital electronics, leading to technological and social change, and affecting economic growth, despite recent slowdowns in semiconductor advancement." - took 2.88s

- Falconsai/text_summarization : "Moore's law is an observation and projection of a historical trend." - took 2.69s

- adasnew/t5-small-xsum : "Moore's law of eponymy has been used to predict the future of the semiconductor industry over the next 10 years." - took 1.62s

- t5-small : "Moore's law is an observation and projection of a historical trend." - took 2.69s

- t5-base : "The number of transistors in an integrated circuit (ic) doubles about every two years." - took 7.22s
```
