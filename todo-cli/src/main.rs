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
                        todo::add_todo(&mut todos, value.clone());
                        save(&todos)?;
                        println!("\x1b[32m(Added: {}){RESET} {value}", todos.len())
                    } else {
                        println!("\x1b[31m(Er) No title specified!{RESET}\nTry help to see usage");
                    }
                }
                "clear" => {
                    todo::clear(&mut todos);
                    save(&todos)?;
                    println!("\x1b[33m(Cleared) Todo List{RESET}")
                }
                "mark" => {
                    let value = program_args.next();
                    if let Some(val) = &value
                        && let Ok(id) = val.parse::<usize>()
                        && let Ok(mark) = todo::mark_todo(&mut todos, id)
                    {
                        save(&todos)?;
                        if mark {
                            println!("\x1b[32m(Marked) x ID:{id} {RESET} as completed")
                        } else {
                            println!("\x1b[33m(Unmarked) - ID:{id} {RESET} as uncompleted")
                        }
                    } else {
                        println!("\x1b[31m(Er) Invalid ID:{}!{RESET}", value.unwrap());
                    }
                }
                "delete" => {
                    let value = program_args.next();
                    if let Some(val) = &value
                        && let Ok(id) = val.parse::<usize>()
                        && let Ok(_) = todo::delete_todo(&mut todos, id)
                    {
                        save(&todos)?;
                        println!("\x1b[33m(Deleted) ID:{id}{RESET}");
                        return Ok(());
                    } else {
                        println!("\x1b[31m(Er) Invalid ID:{}!{RESET}", value.unwrap());
                    }
                }
                "list" => todo::list(&todos),
                _ => println!("\x1b[34m{HELP_DIALOG}{RESET}"),
            }
        }
    } else {
        println!("\x1b[34m{HELP_DIALOG}{RESET}");
    }
    Ok(())
}
