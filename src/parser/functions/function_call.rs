use crate::parser::functions::built_in::{sina_input, sina_print};
use crate::parser::variables::variable_assignment::get_value;

static BUILT_IN_FUNCTIONS: &[&str] = &["println", "input", "int_input"];

pub fn handle_function_call(file_name: &str, expr: &str, call_index: i32) -> (String, String) {
    let arguments: Vec<String> = get_call_arguments(expr, call_index);
    let evaluated_arguments: Vec<String> = evaluate_arguments(file_name, arguments);
    let function_name = expr[..call_index as usize].to_string();

    if BUILT_IN_FUNCTIONS.contains(&function_name.as_str()) {
        if &function_name == "println" {
            sina_print(evaluated_arguments);
            return ("".to_string(), "null".to_string());
        } else if &function_name == "input" {
            if evaluated_arguments.len() != 1 {
                eprint!("Input function requires 1 argument");
                std::process::exit(1);
            }
            return (sina_input(evaluated_arguments.join("").as_str()), "str".to_string());
        } else if &function_name == "int_input" {
            if evaluated_arguments.len() != 1 {
                eprint!("Input function requires 1 argument");
                std::process::exit(1);
            }
            return (
                sina_input(evaluated_arguments.join("").as_str()).trim_end_matches("'").trim_start_matches("'").to_string(),
                "int".to_string()
            );
        }
    }
    eprint!("Cannot find function in scope {}\n", function_name);
    std::process::exit(-1);
}

pub fn get_call_arguments(expr: &str, call_index: i32) -> Vec<String> {
    let un_split_arguments = get_un_split_call_arguments(expr, call_index);
    let mut arguments: Vec<String> = Vec::new();
    let mut open_bracket_number: usize = 0;
    let mut closed_bracket_number: usize = 0;
    let mut in_function = false;
    let mut in_str = false;
    let mut last_splitter_index: usize = 0;

    for (i, c) in un_split_arguments.chars().enumerate() {
        if c.to_string() == "(" {
            open_bracket_number += 1;
        };
        if c.to_string() == ")" {
            closed_bracket_number += 1;
        };
        if c.to_string() == "'" {
            in_str = !in_str;
        };
        if closed_bracket_number != open_bracket_number {
            in_function = true;
        } else {
            in_function = false;
        }
        if c.to_string() == "," && !in_function && !in_str {
            arguments.push(un_split_arguments[last_splitter_index..i].trim_start_matches(" ").to_string());
            last_splitter_index = i+1;
        }
    }

    arguments.push(un_split_arguments[last_splitter_index..].trim_start_matches(" ").to_string());

    arguments
}

pub fn evaluate_arguments(file_name: &str, arguments: Vec<String>) -> Vec<String> {
    arguments
        .iter()
        .map(|arg| get_value(file_name, arg).0.clone()) // take the first element of tuple
        .collect()
}

fn get_un_split_call_arguments(expr: &str, call_index: i32) -> String {
    let sliced_expr: String = expr[call_index as usize..].to_string();
    let mut open_bracket_number: usize = 0;
    let mut closed_bracket_number: usize = 0;
    let mut un_split_arguments: String = sliced_expr.clone();
    for (index, c) in sliced_expr.chars().enumerate() {
        if c == '(' {
            open_bracket_number += 1;
        };
        if c == ')' {
            closed_bracket_number += 1;
            if open_bracket_number == closed_bracket_number {
                un_split_arguments = sliced_expr[..index].to_string();
                break;
            }
        }
    }
    un_split_arguments[1..].to_string()
}
