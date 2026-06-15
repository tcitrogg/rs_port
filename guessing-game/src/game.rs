use crate::RESET;
use std::io::{self, Write};

pub struct Stats {
    pub game_played: u32,
    pub game_won: u32,
    pub total_guesses: u32,
    pub best_score: Option<u32>,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            game_played: 0,
            game_won: 0,
            total_guesses: 0,
            best_score: None,
        }
    }

    pub fn play_game(&mut self, max_guesses: u32, correct_number: u32) {
        self.game_played += 1;
        let mut no_of_guesses: u32 = 1;

        while max_guesses >= no_of_guesses {
            print!("Attempts \x1b[34m({no_of_guesses}/{max_guesses}){RESET} < ");
            let mut input = String::new();

            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut input)
                .expect("\x1b[31m(Err) Reading input{RESET}");

            match input.trim() {
                "exit!" | "q!" => return,
                _ => {
                    if let Ok(value) = input.trim().parse::<u32>() {
                        if value == correct_number {
                            self.game_won += 1;
                            println!("\x1b[32m:) You won in {no_of_guesses} guesses!{RESET}");
                            self.total_guesses += no_of_guesses;
                            match self.best_score {
                                Some(best_score) => {
                                    if best_score > no_of_guesses {
                                        self.best_score = Some(no_of_guesses);
                                        println!("\x1b[35m+ New Best Score!{RESET}");
                                    }
                                }
                                None => {
                                    self.best_score = Some(no_of_guesses);
                                    println!("\x1b[35m+ New Best Score!{RESET}");
                                }
                            }
                            return;
                        } else if value > correct_number {
                            println!("\x1b[33m^ Too High{RESET}");
                        } else if value < correct_number {
                            println!("\x1b[34mv Too Low{RESET}");
                        }
                    } else {
                        println!("\x1b[31m(Err) Not a Number{RESET}");
                    }
                }
            }
            no_of_guesses += 1;
        }

        self.total_guesses += no_of_guesses;
        println!("\x1b[32mx( You've exhausted your guesses!{RESET}");
    }

    pub fn print_stats(&self) {
        println!("\n\x1b[34m─── Session Statistics ───{RESET}");
        println!("\x1b[34mGame Played │{}{RESET}", self.game_played);
        println!("\x1b[32mGame Won    │{}{RESET}", self.game_won);
        println!(
            "\x1b[34mWin Rate    │{}%{RESET}",
            (self.game_won as f64 / self.game_played as f64) * 100_f64
        );

        if let Some(best_score) = self.best_score {
            println!("\x1b[32mBest Score  │{}{RESET}", best_score);
            println!(
                "\x1b[34mAvg Guessed │{}{RESET}",
                self.total_guesses / self.game_played
            );
        } else {
            println!("\x1b[32mBest Score  │{}{RESET}", 0);
            println!("\x1b[34mAvg Guesses │{}{RESET}", 0.0);
        }
    }
}
