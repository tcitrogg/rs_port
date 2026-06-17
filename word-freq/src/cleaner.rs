pub fn clean(file_content: &String) -> Vec<String> {
    let mut result = vec![];
    for each_part in file_content.split_whitespace() {
        if each_part.len() != 0 {
            result.push(
                each_part
                    .chars()
                    .filter(|c| c.is_alphabetic())
                    .collect::<String>()
                    .to_lowercase(),
            );
        }
    }
    result
}
