# Pixie

CLI which generate a random image on a square grid based on a given word

Here an example of the sentence "hello word" on 10 pixel grid:

```
┌──────────────────────┐
│                      │
│   ████  ████  ████   │
│ ██  ██        ██  ██ │
│ ██████        ██████ │
│ ██  ████████████  ██ │
│ ██  ████    ████  ██ │
│                      │
│       ████████       │
│   ████  ████  ████   │
│     ████████████     │
└──────────────────────┘
```

CLI usage is show with `--help`

```
Usage: pixie [OPTIONS] <WORD>

Arguments:
  <WORD>  word used as a base value to generate the image

Options:
  -o, --output <OUTPUT>  format of the generated image (term=ascii characters, png=png file) [default: term]
  -s, --size <SIZE>      size of the pixel grid [default: 10]
  -f, --file <FILENAME>  file where the image should be written. '-' is used to mean stdout. [default: -]
  -h, --help             Print help
  -V, --version          Print version
```
