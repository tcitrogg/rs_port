extern crate regex;
use regex::Regex;

pub enum LineType {
    H(u8, String),
    Paragraph(String),
    OrderedList(String),
    UnorderedList(String),
    CodeBlock { language: String, code: String },
    Image { alt_text: String, path: String },
    HorizontalLine,
}

/// ← line classification, block-level parsing
pub fn classify_line(line: &str) -> LineType {
    let header_re = Regex::new(r"^(#{1,6})\s+(.*)\n$").unwrap();

    if let Some(cg) = header_re.captures(line) {
        let header_num = cg[1].len() as u8;
        LineType::H(header_num, cg[2].to_string())
    } else {
        LineType::Paragraph(line.to_string())
    }
}
