pub enum InlineType {
    Bold(String),
    Italic(String),
    Strikethrough(String),
    Code(String),
    Link { alt_text: String, path: String },
    Image { alt_text: String, path: String },
}

/// ← inline transformations (bold, links, etc.)
pub fn inline_match(line: &str) {}
