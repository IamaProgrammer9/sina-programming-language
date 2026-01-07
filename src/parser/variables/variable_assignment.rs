use crate::{get_variable, variable_exists, Value, Variable};
use crate::parser::variables::variable_parser;

pub fn handle_variable_assignment_expression(file_name: &str, expr: &str, equal_index: i32) {
    // Getting the name of the variable to assign the value to
    let var_name: String = get_expression_variable_name(expr, equal_index).to_string();
    let expression_value_str: String = get_expression_value_str(expr, equal_index).to_string();
    let supposed_value_type: String = get_supposed_expression_value_type(file_name, &expression_value_str);
    let expression_value_parts = get_expression_value_parts(&expression_value_str.trim_end_matches(";"));

    evaluate_expression_value(file_name, expression_value_parts, &supposed_value_type);

    // Checking if it exists in the active file
    if !variable_exists(file_name, &var_name) {
        eprintln!("Cannot assign value to non existing variable {}", var_name);
        std::process::exit(1);
    }
}

fn get_expression_variable_name(expr: &str, equal_index: i32) -> &str {
    expr[0..equal_index as usize].trim()
}

fn get_expression_value_str(expr: &str, equal_index: i32) -> &str {
    expr[equal_index as usize + 1..].trim()
}

fn get_expression_value(value: &str) -> &str {
    let expression_parts = value.split_whitespace().collect::<Vec<&str>>();
    let mut value_type: &str = "";
    let first_part = expression_parts[0];

    for (index, part) in expression_parts.iter().enumerate() {
        // Handling numbers

    }
    ""
}

fn get_expression_value_parts(value: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut last = 0;

    for (i, c) in value.char_indices() {
        if matches!(c, '+' | '-' | '*' | '/') {
            // push value before operator
            let part = value[last..i].trim();
            if !part.is_empty() {
                parts.push(part);
            }

            // push operator itself
            let op_end = i + c.len_utf8();
            parts.push(&value[i..op_end]);

            // move last after operator
            last = op_end;

            // skip spaces
            while last < value.len() && value[last..].starts_with(' ') {
                last += 1;
            }
        }
    }

    // push final value
    let part = value[last..].trim();
    if !part.is_empty() {
        parts.push(part);
    }

    parts
}

fn evaluate_expression_value(file_name: &str, value_parts: Vec<&str>, supposed_value_type: &str) {
    let mut edit_value_parts: Vec<String> = value_parts.iter().map(|s| s.to_string()).collect();
    for (i, c) in value_parts.iter().enumerate() {
        if matches!(*c, "+" | "-" | "*" | "/") {
            if i == 0 {
                eprintln!("Cannot perform operation to non existing variable");
                std::process::exit(1);
            };
            if *c == "+" {
                let (first_value, first_value_type) = get_value(file_name, &edit_value_parts[i-1]);
                let (second_value, second_value_type) = get_value(file_name, &edit_value_parts[i+1]);
                let mut added: String = "".to_string();
                // Validation
                if &first_value_type != &second_value_type {
                    eprintln!("Cannot perform operation to different value types");
                    std::process::exit(1);
                };
                if first_value_type != supposed_value_type {
                    eprintln!("Expected type {} but got {}", supposed_value_type, first_value_type);
                    std::process::exit(1);
                }
                if first_value_type == "str" {
                    added =
                        format!("'{}{}'",
                                first_value.replace("'", ""),
                                second_value.replace("'", "")
                        );
                }
                edit_value_parts.remove(i + 1);
                edit_value_parts.remove(i);
                edit_value_parts.remove(i - 1);

                println!("Added {}", added);
                edit_value_parts.insert(i - 1, added);
                println!("{}", edit_value_parts.len());

                // Removing the three items (before and after)
            }
        } else {
            // operand
        }
    }


    // edit_value_parts
}

fn get_value(file_name: &str, value_part: &str) -> (String, String) {
    if starts_with_number(value_part) {
        if value_part.contains('.') {
            (value_part.to_string(), "float".to_string())
        } else {
            (value_part.to_string(), "int".to_string())
        }
    } else if value_part.starts_with("'") {
        (value_part.to_string(), "str".to_string())
    } else {
        let variable = get_variable(file_name, value_part);
        if let Some(var) = variable {
            (var.value_as_string(), var.value_type.clone())
        } else {
            eprintln!(
                "Cannot assign variable to non existing variable {}",
                file_name
            );
            std::process::exit(1);
        }
    }
}

fn get_supposed_expression_value_type(
    file_name: &str,
    value: &str
) -> String {
    let expression_parts = value.split_whitespace().collect::<Vec<&str>>();
    let first_part = &expression_parts[0].trim().replace(";", "");

    if starts_with_number(first_part) {
        if first_part.contains('.') {
            "float".to_string()
        } else {
            "int".to_string()
        }
    } else if first_part.starts_with('\'') {
        "string".to_string()
    } else {
        let variable = get_variable(file_name, first_part);
        if let Some(var) = variable {
            var.value_type.clone()
        } else {
            "unknown".to_string()
        }
    }
}

fn starts_with_number(s: &str) -> bool {
    s.chars().next().map_or(false, |c| c.is_ascii_digit())
}
