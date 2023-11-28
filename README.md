# Experiment

Extreme summarization of web pages using OpenAI and various local models.

Results on an Apple M1:

```
Models loaded
t5-small-xsum => size: 230.81 MiB
t5-small => size: 230.81 MiB
t5-base => size: 850.31 MiB

Working on https://en.wikipedia.org/wiki/Sandman
Text length: 9577
- openai : "The Sandman is a mythical figure from European folklore who uses magical sand to help people sleep and have beautiful dreams, appearing in many children's stories and media." - took 2.55s

- t5-small-xsum : "The sandman is a mythical character in many children's stories and books." - took 2.44s

- t5-small : "The sandman is a mythical character in many children's stories and books." - took 2.30s

- t5-base : "The sandman is a mythical character in many children's stories and books." - took 7.27s

Working on https://en.wikipedia.org/wiki/Named-entity_recognition
Text length: 9705
- openai : "Named-Entity Recognition (NER) aims to identify and categorize named entities in unstructured text into pre-defined categories, with systems for English achieving near-human performance and focusing on precision, recall, and F1 score measures for evaluation." - took 2.08s

- t5-small-xsum : "Ner is a subtask of information extraction that seeks to locate and classify named entities." - took 2.68s

- t5-small : "Ner is a subtask of information extraction that seeks to locate and classify named entities." - took 2.64s

- t5-base : "Named-entity recognition (ner) is a subtask of information extraction." - took 7.27s

Working on https://en.wikipedia.org/wiki/Firefox
Text length: 50846
- openai : "Mozilla Firefox is a popular open-source web browser developed by the Mozilla Foundation, using the Gecko rendering engine to display web pages and incorporating new technology under the code name "Quantum" to promote parallelism and a more intuitive user interface." - took 2.60s

- t5-small-xsum : "Mozilla firefox is a free and open-source web browser developed by the mozilla foundation and its subsidiary, the mozilla corporation." - took 2.62s

- t5-small : "Mozilla firefox is a free and open-source web browser developed by the mozilla foundation and its subsidiary, the mozilla corporation." - took 2.67s

- t5-base : "Mozilla firefox, or simply firefox, is a free and open-source[11] web browser." - took 7.08s

Working on https://en.wikipedia.org/wiki/Mozilla
Text length: 27948
- openai : "Mozilla is a free software community that develops and supports products like the Firefox web browser, Thunderbird email client, Bugzilla bug tracking system, and the Gecko layout engine, promoting exclusively free software and open standards." - took 2.22s

- t5-small-xsum : "The mozilla community uses, develops, publishes and supports mozilla products." - took 2.52s

- t5-small : "The mozilla community uses, develops, publishes and supports mozilla products." - took 2.44s

- t5-base : "Mozilla is a free software community founded in 1998 by members of netscape." - took 7.05s

Working on https://en.wikipedia.org/wiki/Moore%27s_law
Text length: 27590
- openai : "Moore's law states that the number of transistors on an integrated circuit doubles about every two years, driving growth in digital electronics and contributing to technological and social change, productivity, and economic growth; however, the law's future applicability is debated as semiconductor advancement has slowed industry-wide since around 2010." - took 2.52s

- t5-small-xsum : "Moore's law is an observation and projection of a historical trend." - took 2.76s

- t5-small : "Moore's law is an observation and projection of a historical trend." - took 2.59s

- t5-base : "The number of transistors in an integrated circuit (ic) doubles about every two years." - took 7.15s
```
