use crate::{get_variable, set_variable_value, variable_exists, Value, Variable, GLOBAL_TREE, get_variable_type, is_constant};
use crate::parser::variables::variable_parser;
use crate::parser::variables;
use crate::parser::variables::variable_parser::create_value;

pub fn handle_variable_assignment_expression(file_name: &str, expr: &str, equal_index: i32) {
    // Getting the name of the variable to assign the value to
    let var_name: String = get_expression_variable_name(expr, equal_index).to_string();
    // Checking if it exists in the active file
    if !variable_exists(file_name, &var_name) {
        eprintln!("Cannot assign value to non existent variable {}", var_name);
        std::process::exit(1);
    };

    // Checking if it's constant
    let is_constant = is_constant(file_name, &var_name);
    if is_constant {
        eprintln!("Cannot assign value to constant value {}", var_name);
        std::process::exit(1);
    }

    // Matching the types
    let var_type: String = get_variable_type(file_name, &var_name);
    let expression_value_str: String = get_expression_value_str(expr, equal_index).to_string();
    let supposed_value_type: String = get_supposed_expression_value_type(file_name, &expression_value_str);
    if var_type != supposed_value_type {
        eprintln!("Cannot assign value of type {} to variable {} of type {}", supposed_value_type, var_name, var_type);
        std::process::exit(1);
    };

    // Evaluating
    let expression_value_parts: Vec<&str> = get_expression_value_parts(&expression_value_str.trim_end_matches(";"));
    let evaluated_value: Value = evaluate_expression_value(file_name, expression_value_parts, &supposed_value_type);

    set_variable_value(file_name, &var_name, evaluated_value);
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

fn evaluate_expression_value(file_name: &str, value_parts: Vec<&str>, supposed_value_type: &str) -> Value {
    let mut edit_value_parts: Vec<String> = value_parts.iter().map(|s| s.to_string()).collect();
    let mut i = 0;
    let mut added: String = "".to_string();
    while i < edit_value_parts.len() {
        let c = &edit_value_parts[i];
        if matches!(c.as_str(), "+" | "-" | "*" | "/") {
            if i == 0 {
                eprintln!("Cannot perform operation on non-existing variable");
                std::process::exit(1);
            };
            let (first_value, first_value_type) = get_value(file_name, &edit_value_parts[i-1]);
            let (second_value, second_value_type) = get_value(file_name, &edit_value_parts[i+1]);
            // Validation
            if &first_value_type != &second_value_type {
                eprintln!("Cannot perform operation on different value types");
                std::process::exit(1);
            };
            if first_value_type != supposed_value_type {
                eprintln!("Expected type {} but got {}", supposed_value_type, first_value_type);
                std::process::exit(1);
            }
            // Plus operator (Works for strings & integers)
            if *c == "+" {
                // Handling string addition
                if first_value_type == "str" {
                    added =
                        format!("'{}{}'",
                                first_value.replace("'", ""),
                                second_value.replace("'", "")
                        );
                edit_value_parts.remove(i + 1);
                edit_value_parts.remove(i);
                edit_value_parts.remove(i - 1);
                edit_value_parts.insert(i - 1, added);

                // Handling integer addition
                } else if first_value_type == "int" {
                    let first_value_str = first_value.to_string();
                    let second_value_str = second_value.to_string();
                    let numbers_added =
                        first_value_str.parse::<i32>().unwrap()
                        + second_value_str.parse::<i32>().unwrap();
                    edit_value_parts.remove(i + 1);
                    edit_value_parts.remove(i);
                    edit_value_parts.remove(i - 1);
                    edit_value_parts.insert(i - 1, numbers_added.to_string());
                }

                i = 0;
                continue;
            } else if *c == "-" {
                if first_value_type != "int" {
                    eprintln!("Cannot perform subtraction operation on variables on variable of type int");
                    std::process::exit(1);
                };
                let first_value_str = first_value.to_string();
                let second_value_str = second_value.to_string();
                let subtracted_numbers = first_value_str.parse::<i32>().unwrap() -
                                              second_value_str.parse::<i32>().unwrap();

                edit_value_parts.remove(i + 1);
                edit_value_parts.remove(i);
                edit_value_parts.remove(i - 1);
                edit_value_parts.insert(i - 1, subtracted_numbers.to_string());

                i = 0;
                continue;
            } else if *c == "*" {
                if first_value_type != "int" {
                    eprintln!("Cannot perform multiplication operation on variables on variable of type int");
                    std::process::exit(1);
                };
                let first_value_str = first_value.to_string();
                let second_value_str = second_value.to_string();
                let multiplied_numbers = first_value_str.parse::<i32>().unwrap() *
                    second_value_str.parse::<i32>().unwrap();

                edit_value_parts.remove(i + 1);
                edit_value_parts.remove(i);
                edit_value_parts.remove(i - 1);
                edit_value_parts.insert(i - 1, multiplied_numbers.to_string());

                i = 0;
                continue;
            } else if *c == "/" {
                if first_value_type != "int" {
                    eprintln!("Cannot perform multiplication operation on variables on variable of type int");
                    std::process::exit(1);
                };
                let first_value_str = first_value.to_string();
                let second_value_str = second_value.to_string();
                let divided_number = first_value_str.parse::<i32>().unwrap() /
                    second_value_str.parse::<i32>().unwrap();

                edit_value_parts.remove(i + 1);
                edit_value_parts.remove(i);
                edit_value_parts.remove(i - 1);
                edit_value_parts.insert(i - 1, divided_number.to_string());

                i = 0;
                continue;
            } else {
                i = 1;
            }
        }
        i += 1;
    }

    let value = create_value(edit_value_parts.join("").as_str(), supposed_value_type);
    value
}

pub fn get_value(file_name: &str, value_part: &str) -> (String, String) {
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
                value_part
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
