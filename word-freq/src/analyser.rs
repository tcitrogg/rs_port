use std::collections::HashMap;

use crate::RESET;

pub fn analyse(vec_content: Vec<String>) -> HashMap<String, u32> {
    let mut store: HashMap<String, u32> = HashMap::new();
    for each_string in vec_content {
        *store.entry(each_string).or_insert(0) += 1_u32;
    }
    store
}

pub fn stats(store: HashMap<String, u32>, files: Vec<String>) -> String {
    let mut result = String::from("Word Frequency Analysis");
    
    if files.len() > 1 {
        result += "\nFile(s): ";
        result += files.iter().fold(String::new(), |acc, s| acc+" "+s).as_str();
    } else {
        result += format!("File(s): {}", files[0]).as_str();
    }

    // top 5 words
    let sorted = store.into_iter().collect::<Vec<(String, u32)>>();
    sorted.sort_by(|a, b|b.1.cmp(&a.1));
    for each_word in top_five{
        println!("{}", each_word)
    }

    result
}

pub fn save_report(store: HashMap<String, u32>, file_name: String) -> String{

    std::fs::write(path, contents)
    file_name
}
