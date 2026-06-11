use std::io::{self, Write};

static HELP_DIALOG: &str = r#"
help!    - show this dialog.
history! - show history.
exit!    - quit.
"#;

static RESET: &str = "\x1b[0m";

fn scan_expr(expr: &String) -> (char, Vec<String>) {
    let mut operator = ' ';
    if expr.contains("+") {
        operator = '+';
    } else if expr.contains("-") {
        operator = '-';
    } else if expr.contains("*") {
        operator = '*';
    } else if expr.contains("/") {
        operator = '/';
    }

    let vec_operands = expr
        .split(operator)
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    (operator, vec_operands)
}

fn parse_cal(expr: &String) {
    let trimed_expr = expr.trim().split(" ").collect::<Vec<&str>>();
    if let Ok(left_operand) = trimed_expr[0].parse::<f64>() {
        if let Ok(right_operand) = trimed_expr[2].parse::<f64>() {
            match trimed_expr[1] {
                "+" => println!("\x1b[32m>{RESET} {}", left_operand + right_operand),
                "-" => println!("\x1b[32m>{RESET} {}", left_operand - right_operand),
                "*" => println!("\x1b[32m>{RESET} {}", left_operand * right_operand),
                "/" => {
                    if right_operand == 0.0 {
                        println!("\x1b[31m(ERR): Division by zero is not allowed.{}", RESET);
                    } else {
                        println!("\x1b[32m>{RESET} {}", left_operand / right_operand)
                    }
                }
                // "^" => println!("Result: {}", left_operand.pow(right_operand)),
                _ => println!("\x1b[31m(ERR): Invalid Operator!{RESET}"),
            }
        };
    } else {
        println!("\x1b[31m(ERR): Invalid Operand!{RESET}")
    }
}

fn main() {
    let mut history: Vec<String> = Vec::new();
    println!("Welcome to Rust Calculator!\n\x1b[34m[Type help! for help dialog]{RESET}");
    println!("Enter expression (e.g - 8 + 15)");
    let mut running = true;
    while running {
        let mut expr = String::new();

        print!("\x1b[33m<{RESET} ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut expr).unwrap();
        match expr.trim() {
            "help!" => println!("{}", HELP_DIALOG),
            "history!" => println!("{:#?}", history),
            "exit!" => {
                running = false;
            }
            _ => {
                parse_cal(&expr);
                history.push(expr.trim().to_string())
            }
        }
    }
}
