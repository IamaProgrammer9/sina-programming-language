use crate::add_to_global_tree;

pub mod variable_parser;

pub fn parse(file: Vec<String>) {
    for line in file {
        let trimmed_line: &str = line.trim();
        let mut is_valid = false;

        // Skip empty lines and comments
        if trimmed_line.is_empty() || trimmed_line.starts_with('#') {
            continue;
        }

        // Variable detection
        if first_word(trimmed_line) == "let" {
            variable_parser::parse_variable_expression(trimmed_line);
            is_valid = true;
        }

        if !is_valid {
            eprintln!("Invalid syntax at: {}", trimmed_line);
            std::process::exit(1);
        }
    }
}

fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}