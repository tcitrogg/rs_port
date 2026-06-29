```
md-convert/
├── Cargo.toml
└── src/
    ├── main.rs         ← CLI entry, file I/O
    ├── parser.rs       ← line classification, block-level parsing
    ├── inline.rs       ← inline transformations (bold, links, etc.)
    └── renderer.rs     ← HTML assembly and boilerplate
tests/
    └── conversion_tests.rs
```

# heading 1
## heading 2
### heading 3
#### heading 4
##### heading 5
###### heading 6


**this is for a bold text**
*this is a italic text*
~~this is strikethrough~~

- first
- second
- third

1. first
2. second
3. third

this was built with `rust`

this is a link [alt text](url)

--- 


## TODO
- [ ] `# Heading 1` through `###### Heading 6`
- [ ] `*bold**` and `italic*` and `~~strikethrough~~`
- [ ] Unordered lists `item`
- [ ] Ordered lists `1. item`
- [ ] Inline code ``code``
- [ ] Fenced code blocks `````
- [ ] `link text` → `<a href="url">link text</a>`
- [ ] `!alt` → `<img src="url" alt="alt">`
- [ ] Blank lines between paragraphs → `<p>` tags
- [ ] Horizontal rules `--`
