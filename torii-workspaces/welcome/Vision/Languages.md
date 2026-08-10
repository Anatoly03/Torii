# Constructed Languages

The language component is used to document languages used in the world. These can either be constructed languages, or language dialects. A language can come with its' own alphabet (script), vocabulary and grammar.

```
├── Kirlin
│   ├── Kirlin Script
│   │   ├── Rune A
│   │   ├── Rune A (Dotted)
│   │   └── ...
│   ├── Exhaustive Guide to Kirlin Grammar
│   └── Vocabulary
└── Vourian
    ├── Void Symbols
    │   ├── Life Symbol
    │   ├── Void Symbol
    │   └── ...
    └── The Magic System of Void Spells
```

To create new symbols (= script, letters) you would create new records with an SVG image component attached to it (e.g. "Letter A"). Grammar guides could utilize regular articles to create powerful explanations of how the language is defined.



An implementation question is how the language could be reused in the article component to insert custom scripts, but that's a technical question for later. I see even the ability to create custom keybinds so you can quickly write in your own language.



Optionally in the far future we could create a language model-powered translation engine, so users can translate their own constructed language into English, and vice versa, but the ethics of how to implement this need to be discussed when the project grows bigger. For example, AI should be opt-in and not opt-out and never replace human creativity, only assist.



The components' file pattern should be `<entity>.language.yaml`.

