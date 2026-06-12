use std::{env::args, error::Error, usize};

use crate::storage::{load, save};
mod storage;
mod todo;

static RESET: &str = "\x1b[0m";
static HELP_DIALOG: &str = r#"
[x] TODO
- help       - show this dialog
- add <text> - adds a new task
- list       - displays all tasks with their status
- mark 2     - marks task #2 as complete, if it is marked it will unmark it
- delete 3   - removes task #3
- clear      - removes all completed tasks
"#;

fn main() -> Result<(), Box<dyn Error>> {
    let mut program_args = args();
    // 1st -> program, 2nd -> action, 3rd -> value
    if program_args.len() >= 2 {
        let _program_bin = program_args.next();
        if let Some(action) = program_args.next() {
            let mut todos = load();
            //let value = program_args.next();
            match action.as_str() {
                // action
                "add" => {
                    if let Some(value) = program_args.next() {
                        todo::add_todo(&mut todos, value);
                        save(&todos)?;
                    } else {
                        println!("\x1b[33m(Er) No title specified!{RESET}\nTry help to see usage");
                    }
                }
                "clear" => {
                    todo::clear(&mut todos);
                    save(&todos)?;
                }
                "mark" => {
                    if let Some(value) = program_args.next() {
                        if let Ok(id) = value.parse::<usize>() {
                            todo::mark_todo(&mut todos, id);
                            save(&todos)?;
                        } else {
                            println!("\x1b[33m(Er) Invalid ID!{RESET}\n");
                        }
                    }
                }
                "delete" => {
                    if let Some(value) = program_args.next() {
                        if let Ok(id) = value.parse::<usize>() {
                            todo::delete_todo(&mut todos, id);
                            save(&todos)?;
                        } else {
                            println!("\x1b[33m(Er) Invalid ID!{RESET}\n");
                        }
                    }
                }
                "list" => println!("\x1b[35m{:#?}{RESET}", todos),
                _ => println!("\x1b[34m{HELP_DIALOG}{RESET}"),
            }
        }
    } else {
        println!("\x1b[34m{HELP_DIALOG}{RESET}");
    }
    Ok(())
}
