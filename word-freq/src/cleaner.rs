pub fn clean(file_content: &String) -> Vec<String> {
    file_content
        .split_whitespace()
        .filter(|c| c.chars().all(|c| c.is_alphabetic()))
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
}
