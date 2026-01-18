use crate::{add_to_global_tree, Value};
use crate::parser::variables::variable_assignment::evaluate_expression_value;

mod variables;
mod functions;
mod conditionals;

pub fn parse(file_name: &str, file: Vec<String>) {
    let mut skip_lines: Vec<usize> = Vec::new();
    let mut expected_else: Vec<usize> = Vec::new();
    for (i, line) in file.iter().enumerate() {
        let trimmed_line: &str = line.trim();
        let mut is_valid = false;

        // Skip empty lines and comments
        if trimmed_line.is_empty() || trimmed_line.starts_with('#') || skip_lines.contains(&i) {
            continue;
        }
        // Variable detection
        if first_word(trimmed_line) == "let" || first_word(trimmed_line) == "const" {
            variables::variable_parser::parse_variable_expression(file_name, trimmed_line);
            is_valid = true;
        } else if first_word(trimmed_line) == "if" {
            let conditional_end: usize = get_multiline_expression_end(file.clone(), i);

            let slice = &file[i..conditional_end];
            let lines: Vec<&str> = slice
                .iter()
                .map(|s| s.as_str())
                .collect();
            let is_evaluated = conditionals::handler::evaluate_conditional(&file_name, lines);

            // Expecting an else if it isn't evaluated
            if !is_evaluated {
                expected_else.push(conditional_end+1);
            }

            // Making the parser skip conditional lines
            skip_lines.extend(i..conditional_end);
            is_valid = true;
        } else if first_word(trimmed_line) == "}" {
            is_valid = true;
            if trimmed_line.trim_start_matches("}").trim().starts_with("else") {
                let conditional_end: usize = get_multiline_expression_end(file.clone(), i);
                if expected_else.contains(&i) {

                } else {

                }
            }
        } else {
            let (expr_type, index) = get_expression_type(trimmed_line);
            if expr_type == "variable_assignment" {
                // println!("{}", &trimmed_line[index as usize..]);
                variables::variable_assignment::handle_variable_assignment_expression(file_name, trimmed_line, index);
                is_valid = true;
            } else if expr_type == "function_call" {
                functions::function_call::handle_function_call(file_name, trimmed_line, index);
                is_valid = true;
            }
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

fn get_expression_type(expr: &str) -> (&str, i32) {
    let mut expression_type: &str = "";
    let mut expression_start: i32 = 0;
    for (i, c) in expr.chars().enumerate() {
        if c == '=' {
            expression_type = "variable_assignment";
            expression_start = i as i32;
            break;
        };
        if c == '(' {
            expression_type = "function_call";
            expression_start = i as i32;
            break;
        }
    }
    (expression_type, expression_start)
}

fn get_multiline_expression_end(file: Vec<String>, index: usize) -> usize {
    let mut closed_braces_count: usize = 0;
    let mut open_braces_count: usize = 1;
    let mut final_index: usize = 0;
    for (i, c) in file.iter().enumerate() {
        if c.trim().trim_end_matches(";") == "}" {
            closed_braces_count += 1;
        } else if c.trim().trim_end_matches(";") == "{" {
            open_braces_count += 1;
        }
        if closed_braces_count == open_braces_count {
            final_index = i;
            break;
        }
    };
    if final_index == 0 {
        eprintln!("Function end could not be found");
        std::process::exit(1);
    };
    final_index
}

pub fn get_statement_value(file_name: &str, expression: &str) -> Value {
    let supposed_value_type: String = variables::variable_assignment::get_supposed_expression_value_type(file_name, &expression);
    let expression_value_parts: Vec<&str> = variables::variable_assignment::get_expression_value_parts(expression);
    let evaluated_value: Value = evaluate_expression_value(file_name, expression_value_parts, &supposed_value_type);

    evaluated_value
}
