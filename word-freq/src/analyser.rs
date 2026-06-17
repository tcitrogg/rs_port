use std::collections::HashMap;

// use crate::RESET;

pub fn analyse(vec_content: Vec<String>) -> (usize, HashMap<String, u32>) {
    let mut store: HashMap<String, u32> = HashMap::new();
    for each_string in vec_content.clone() {
        *store.entry(each_string.to_lowercase()).or_insert(0) += 1_u32;
    }
    (vec_content.len(), store)
}

pub fn get_top_words(top_words_count: usize, store: &HashMap<String, u32>) -> String {
    let mut result = String::new();

    let mut sorted = store.into_iter().collect::<Vec<(&String, &u32)>>();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));

    let mut new_top_words_count = top_words_count;
    if new_top_words_count > sorted.len() {
        new_top_words_count = sorted.len()
    }
    let selected_top_words = &sorted[0..new_top_words_count];

    //let unique_store_len = store.len();
    let max_word_len = selected_top_words
        .iter()
        .map(|(w, _)| w.len())
        .max()
        .unwrap_or(1);
    result += format!("\n### Top {new_top_words_count} Words:\n").as_str();
    for (id, each_value) in selected_top_words.iter().enumerate() {
        result += format!(
            "{:>3}. {:<max_word_len$} -> {:>4} times | {:>4.1}%\n",
            id + 1,
            each_value.0,
            each_value.1,
            (*each_value.1 as f64 / store.len() as f64) * 100.0
        )
        .as_str();
    }
    result
}

pub fn stats(analysis_info: (usize, HashMap<String, u32>), files: &[String]) -> String {
    let top_words_count: usize = 5;
    let (_, store) = analysis_info;
    let mut result = String::from(
        "## Word Frequency Analysis\n> Common stop words e.g (the, and, a, is, etc.) are excluded\n",
    );

    if files.len() > 1 {
        result += "\n### Files:\n";
        result += files
            .iter()
            .fold(String::new(), |acc, s| acc + "  - " + s + "\n")
            .as_str();
    } else {
        result += format!("\n### File: {}\n", files[0]).as_str();
    }
    // top 5 words
    result += get_top_words(top_words_count, &store).as_str();

    // other stats
    let avg_word_len = &store.iter().map(|w| w.0.len()).sum::<usize>() / store.len();
    result += "\n### Statistics:\n";
    let total_words = store.values().sum::<u32>();
    result += format!("- Total Words    : {}\n", total_words).as_str();
    result += format!("- Unique Words   : {}\n", store.len()).as_str();
    result += format!("- Avg Word Len   : {}\n", avg_word_len).as_str();

    result
}
