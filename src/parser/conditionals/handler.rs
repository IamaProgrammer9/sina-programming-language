use crate::parser::variables::variable_assignment::get_value;
use crate::parser::variables::variable_assignment::evaluate_expression_value;
use crate::parser;
use crate::parser::parse;

pub fn evaluate_conditional(file_name: &str, conditional: Vec<&str>) -> bool {
    let first_line =  conditional[0].to_string();
    let conditional_statement = get_conditional_statement(&first_line);
    let (condition_type, seperator_index, seperator_size) = get_conditional_type(conditional_statement);
    let first_part = conditional_statement[..seperator_index].trim_start();
    let second_part = conditional_statement[seperator_index+seperator_size..].trim_start();
    let first_part_value = parser::get_statement_value(file_name, first_part);
    let second_part_value = parser::get_statement_value(file_name, second_part);
    // Equality (==)
    if condition_type == "equation" {
        if first_part_value == second_part_value {
            let lines: Vec<String> = conditional[1..]
                .iter()
                .map(|s| s.to_string())
                .collect();
            parse(file_name, lines);
            return true;
        } else {
            println!("First part value: {:?}, Second part value: {:?}", first_part_value, second_part_value);
            return false;
        }
    };
    // Inequality (!=)
    if condition_type == "inequality" {
        if first_part_value != second_part_value {
            let lines: Vec<String> = conditional[1..]
                .iter()
                .map(|s| s.to_string())
                .collect();
            parse(file_name, lines);
            return true;
        } else {
            return false;
        }
    }
    // Any operation onwards will need an integer
    let first_value_type = parser::variables::validators::get_value_type(&first_part_value);
    let second_value_type = parser::variables::validators::get_value_type(&second_part_value);
    if first_value_type != "int" || second_value_type != "int" {
        // println!("First part value: {:?}, Second part: {:?}", first_part_value, second_part_value);
        eprint!("Numerical operations cannot be done on type {}, {}", first_value_type, second_value_type);
        std::process::exit(1);
    }
    // Less than (<)
    if condition_type == "less_than" {
        if first_part_value.as_int() < second_part_value.as_int() {
            let lines: Vec<String> = conditional[1..]
                .iter()
                .map(|s| s.to_string())
                .collect();
            parse(file_name, lines);
            return true;
        } else {
            return false;
        }
    }
    // Less than or equal to (<=)
    if condition_type == "less_than_or_equal" {
        if first_part_value.as_int() <= second_part_value.as_int() {
            let lines: Vec<String> = conditional[1..]
                .iter()
                .map(|s| s.to_string())
                .collect();
            parse(file_name, lines);
            return true;
        } else {
            return false;
        }
    }
    // Greater than
    if condition_type == "greater_than" {
        if first_part_value.as_int() > second_part_value.as_int() {
            let lines: Vec<String> = conditional[1..]
                .iter()
                .map(|s| s.to_string())
                .collect();
            parse(file_name, lines);
            return true;
        } else {
            return false;
        }
    }
    // Greater than or equal to (>=)
    if condition_type == "greater_than_or_equal" {
        if first_part_value.as_int() >= second_part_value.as_int() {
            let lines: Vec<String> = conditional[1..]
                .iter()
                .map(|s| s.to_string())
                .collect();
            parse(file_name, lines);
            return true;
        } else {
            return false;
        }
    }
    false
}

fn get_conditional_statement(line: &str) -> &str {
    line.trim_start_matches("if").trim_end_matches(" {")
}

fn get_conditional_type(statement: &str) -> (&str, usize, usize) {
    let mut in_str: bool = false;
    let mut conditional_type: &str = "";
    let mut seperator_index: usize = 0;
    let mut seperator_size: usize = 0;

    for (i, c) in statement.chars().enumerate() {
        if c == '\'' {
            in_str = !in_str;
        }
        if c == '=' && !in_str {
            if statement.chars().nth(i + 1).unwrap() == '=' {
                seperator_index = i;
                seperator_size = 2;
                conditional_type = "equation";
                break;
            } else {
                eprintln!("Invalid equal sign at conditional.");
                std::process::exit(1);
            }
        }
        if c == '!' && !in_str {
            if statement.chars().nth(i + 1).unwrap() == '=' {
                seperator_index = i;
                seperator_size = 2;
                conditional_type = "inequality";
                break;
            } else {
                eprintln!("Invalid equal sign at conditional.");
                std::process::exit(1);
            }
        }
        if c == '<' {
            seperator_index = i;
            if statement.chars().nth(i + 1).unwrap() == '=' {
                seperator_size = 2;
                conditional_type = "less_than_or_equal";
            } else {
                seperator_size = 1;
                conditional_type = "less_than";
            }
            break;
        }
        if c == '>' {
            seperator_index = i;
            if statement.chars().nth(i + 1).unwrap() == '=' {
                seperator_size = 2;
                conditional_type = "greater_than_or_equal";
            } else {
                seperator_size = 1;
                conditional_type = "greater_than";
            }
            break;
        }
    }
    (conditional_type, seperator_index, seperator_size)
}
