extern crate regex;

use regex::Regex;

fn main() {
    let re = Regex::new(r"\*([\w\d\s^*]+)\*").unwrap();
    println!(
        "> > {:?}",
        re.captures("*this is italic* **this is bold**").unwrap()
    );
}
