// use rand::Rng;
use std::io::{self, Write};

mod game;

static RESET: &str = "\x1b[0m";

static HELP_DIALOG: &str = r#"
help! | h! - show this help dialog
exit! | q! - quit program
"#;

fn main() {
    println!("\x1b[32m🎮 Rust Guessing Game{RESET}");

    let mut stats = game::Stats::new();
    let mut running = true;

    const MAX_GUESSES: u32 = 10;
    // let mut no_of_games = 1;
    println!(
        "Guess a number between 1 and 100. You have \x1b[33m<{MAX_GUESSES}>{RESET} attempts.\n"
    );
    let correct_number: u32 = rand::random_range(1..=100);
    stats.play_game(MAX_GUESSES, correct_number);

    while running {
        print!("\x1b[34m\nPlay again? ([y]es/[n]o): {RESET}");
        let mut play_again_input = String::new();

        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut play_again_input)
            .expect("\x1b[31m(Err) Reading input{RESET}");

        match play_again_input.trim().to_lowercase().as_str() {
            "yes" | "y" => {
                println!(
                    "Guess a number between 1 and 100. You have \x1b[33m<{MAX_GUESSES}>{RESET} attempts.\n"
                );
                let correct_number: u32 = rand::random_range(1..=100);
                stats.play_game(MAX_GUESSES, correct_number);
                continue;
            }
            "no" | "n" => {
                running = false;
                stats.print_stats();
            }
            "help" | "h" => println!("\x1b[34m{HELP_DIALOG}{RESET}"),
            "exit" | "q" => {
                return;
            }
            _ => {
                println!("\x1b[31m(Err) Wrong Input {RESET}");
            }
        }
    }
}
