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
    let content = std::fs::read_to_string(path)?;
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
            let mut files_content = String::new();
            for each_file in &program_args[2..] {
                files_content += &read_file(each_file)?;
            }
        }
        "top" => {
            // head
            if program_args.len() != 4 {
                println!("\x1b[34m{HELP_DIALOG}{RESET}");
                return Ok(());
            }
            if let Ok(count) = &program_args[2].parse::<u32>() {
                let file_content = read_file(&program_args[3])?;
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
