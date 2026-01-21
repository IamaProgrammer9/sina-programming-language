use crate::{add_to_global_tree, Value};
use crate::parser::variables::variable_assignment::evaluate_expression_value;

mod variables;
pub mod functions;
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
            let conditional_end: usize = get_multiline_expression_end(&file.clone(), i);

            let slice = &file[i..conditional_end];
            let lines: Vec<&str> = slice
                .iter()
                .map(|s| s.as_str())
                .collect();
            let is_evaluated = conditionals::handler::evaluate_conditional(&file_name, lines);

            // Expecting an else if it isn't evaluated
            if !is_evaluated {
                // println!("I: {}, Conditional end: {}", i, conditional_end);
            expected_else.push(conditional_end);
                // Making the parser skip conditional lines
            }
        skip_lines.extend(i..conditional_end);

            is_valid = true;
        } else if first_word(trimmed_line) == "}" {
            is_valid = true;
            if trimmed_line.trim_start_matches("}").trim().starts_with("else") {
                if !expected_else.contains(&i) {
                    let conditional_end: usize = get_multiline_expression_end(&file.clone(), i);
                    skip_lines.extend(i..conditional_end);
                }
            }
        } else if first_word(trimmed_line).trim_end_matches(";") == "exit" {
            std::process::exit(0);
        } else {
            let (expr_type, index) = get_expression_type(trimmed_line);
            if expr_type == "variable_assignment" {
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

fn get_multiline_expression_end(
    file: &[String],
    start: usize,
) -> usize {
    let mut open = 0;
    let mut close = 0;
    let mut in_str = false;

    for (line_offset, line) in file[start..].iter().enumerate() {
        for c in line.chars() {
            if c == '\'' {
                in_str = !in_str;
            } else if !in_str {
                if c == '{' {
                    open += 1;
                } else if c == '}' && line_offset != 0 {
                    close += 1;
                }
            }

            if open > 0 && open == close {
                return start + line_offset;
            }
        }
    }

    panic!("Unclosed block");
}

pub fn get_statement_value(file_name: &str, expression: &str) -> Value {
    let supposed_value_type: String = variables::variable_assignment::get_supposed_expression_value_type(file_name, &expression);
    let expression_value_parts: Vec<&str> = variables::variable_assignment::get_expression_value_parts(expression);
    let evaluated_value: Value = evaluate_expression_value(file_name, expression_value_parts, &supposed_value_type);

    evaluated_value
}
