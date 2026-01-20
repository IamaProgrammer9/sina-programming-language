use std::fmt::format;
use std::io::{self, Write};
use crate::parser::functions::return_handler;

pub fn sina_print(args: Vec<String>) {
    let mut cleaned_args: Vec<String> = Vec::new();
    for (i, c) in args.iter().enumerate() {
        cleaned_args.push(c.trim_start_matches("'").trim_end_matches("'").to_string());
    }
    println!("{}", cleaned_args.join(""));
}

pub fn sina_input(message: &str) -> String {
    print!("{}", message.trim_start_matches("'").trim_end_matches("'"));

    // 🔑 Make sure the prompt is shown immediately
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let trimmed = input.trim();

    format!("'{}'", trimmed)
}

/// Initializes all built-in functions in the `GLOBAL_FUNCTION_RETURN_TYPES`.
pub fn register_built_in_functions() {
    return_handler::register_function_return_type("println", "str");
    return_handler::register_function_return_type("input", "str");
    return_handler::register_function_return_type("int_input", "int");
}
