extern crate regex;
use regex::Regex;

pub enum LineType {
    H(u8, String),
    OrderedList(String),
    UnorderedList(String),
    CodeBlock { language: String, code: String },
    HorizontalLine,
    Paragraph(String),
}

/// ← line classification, block-level parsing
pub fn classify_line(line: &str) -> LineType {
    let header_re = Regex::new(r"^(#{1,6})\s+(.*)\n$").unwrap();
    let unordered_list_re = Regex::new(r"^-\s(.*)\n$").unwrap();
    let hr_line_re = Regex::new(r"^-{2,3}$").unwrap();

    if let Some(cg) = header_re.captures(line) {
        let header_num = cg[1].len() as u8;
        LineType::H(header_num, cg[2].to_string())
    } else if let Some(cg) = unordered_list_re.captures(line) {
        LineType::UnorderedList(cg[1].to_string())
    } else if hr_line_re.is_match(line) {
        LineType::HorizontalLine
    } else {
        LineType::Paragraph(line.to_string())
    }
}
