use std::{
    io::{self, Write},
    num::ParseFloatError,
};

static HELP_DIALOG: &str = r#"
help!    | m!  - show this dialog.
history! | h!  - show history.
exit!    | q!  - quit.
"#;

static RESET: &str = "\x1b[0m";

fn check_for_operator(expr: &String) -> Option<char> {
    if expr.contains("*") {
        return Some('*');
    } else if expr.contains("/") {
        return Some('/');
    } else if expr.contains("+") {
        return Some('+');
    } else if expr.contains("-") {
        return Some('-');
    }
    None
}

fn cal_tree(expr: &String) -> Result<f64, String> {
    let trimed_expr = expr.trim();
    if let Some(operator) = check_for_operator(&trimed_expr.to_string()) {
        let Some((left_operand, right_operand)) = trimed_expr.split_once(operator) else {
            return Err(format!("\x1b[31m(Er): Invalid Operand!{RESET}"));
        };
        if let Ok(evaluated_left_operand) = cal_tree(&left_operand.trim().to_string())
            && let Ok(evaluated_right_operand) = cal_tree(&right_operand.trim().to_string())
        {
            match operator {
                '+' => return Ok(evaluated_left_operand + evaluated_right_operand),
                '-' => return Ok(evaluated_left_operand - evaluated_right_operand),
                '*' => return Ok(evaluated_left_operand * evaluated_right_operand),
                '/' => {
                    if evaluated_right_operand == 0.0 {
                        return Err(format!(
                            "\x1b[31m(Er): Division by zero is not allowed.{}",
                            RESET
                        ));
                    }
                    return Ok(evaluated_left_operand / evaluated_right_operand);
                }
                // "^" => println!("Result: {}", left_operand.pow(right_operand)),
                _ => return Err(format!("\x1b[31m(Er): Invalid Operator!{RESET}")),
            }
        }
        // return Err(format!("\x1b[31m(Err): RHS: nvalid Expression!{RESET}"));
    }

    if let Ok(value) = expr.parse::<f64>() {
        return Ok(value);
    } else {
        return Err(format!("\x1b[31m(Er): Invalid Expression!{RESET}"));
    }
}

fn main() {
    let mut history: Vec<String> = Vec::new();
    println!("\x1b[33mWelcome to Rust Calculator!\n\x1b[34m[Type help! for help dialog]{RESET}");
    println!("Enter expression (e.g - 8 + 15)");
    let mut running = true;
    while running {
        let mut expr = String::new();

        print!("\x1b[33m<{RESET} ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut expr).unwrap();
        match expr.trim() {
            "help!" | "m!" => println!("\x1b[34m{HELP_DIALOG}{RESET}"),
            "history!" | "h!" => println!("\x1b[35m{:#?}{RESET}", history),
            "exit!" | "q!" => {
                running = false;
            }
            _ => {
                match cal_tree(&expr) {
                    Ok(v) => println!("\x1b[32m(Ok) {v}{RESET} "),
                    Err(e) => println!("{e}"),
                }
                //calculator(&expr);
                history.push(expr.trim().to_string())
            }
        }
    }
}
