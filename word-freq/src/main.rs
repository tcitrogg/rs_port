use std::error::Error;

mod analyser;
mod cleaner;
mod reporter;

static RESET: &str = "\x1b[0m";
static HELP_DIALOG: &str = r#"
help                - show this help dialog
analyse <file>+     - e.g | analyse file.txt  | analyse files or a file and save report in a file
top <count> <file>  - e.g | top 20 report.txt | show the top 20 words in the report.txt
"#;

pub fn read_file(path: &str) -> Result<String, Box<dyn Error>> {
    let content = std::fs::read_to_string(path)
        .expect(format!("\x1b[31m(Err): Reading File: {path}").as_str());
    Ok(content)
}

fn main() -> Result<(), Box<dyn Error>> {
    let program_args = std::env::args().collect::<Vec<String>>();
    let action = &program_args[1];
    match action.as_str() {
        "analyse" => {
            if program_args.len() < 3 {
                println!("\x1b[34m{HELP_DIALOG}{RESET}");
                return Ok(());
            }
            let list_of_files = &program_args[2..];
            let mut files_content = String::new();
            for each_file in list_of_files {
                files_content += &read_file(each_file)?;
            }

            // clean content
            let cleaned_content = cleaner::clean(&files_content);
            // analyse content
            let analysis_info = analyser::analyse(cleaned_content);
            let complete_stats = analyser::stats(analysis_info, list_of_files);
            // save in file
            let saved_report_name = reporter::save_report(complete_stats);
            println!("\x1b[32m(Saved) Report -> {saved_report_name}");
        }
        "top" => {
            // head
            if program_args.len() != 4 {
                println!("\x1b[34m{HELP_DIALOG}{RESET}");
                return Ok(());
            }
            if let Ok(count) = &program_args[2].parse::<usize>() {
                let file_content = read_file(&program_args[3])?;
                // clean content
                let cleaned_content = cleaner::clean(&file_content);
                // analyse content
                let analysis_info = analyser::analyse(cleaned_content);
                let top_words_result = analyser::get_top_words(*count, &analysis_info.1);

                println!("\n## Word Frequency Analysis");
                print!(
                    "\x1b[33m> Common stop words e.g (the, and, a, is, etc.) are excluded{RESET}"
                );
                println!("\x1b[35m{top_words_result}{RESET}");
            } else {
                println!(
                    "\x1b[31m(Err) Invalid integer value for <count>.{RESET}Try `help` to see usage."
                );
            }
        }
        _ => println!("\x1b[34m{HELP_DIALOG}{RESET}"),
    }

    Ok(())
}
