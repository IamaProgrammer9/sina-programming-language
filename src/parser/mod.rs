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
            let conditional_end: usize = get_multiline_expression_end2(file.clone(), i, 0, 0);

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
            skip_lines.extend(i..conditional_end+i);
            }

            is_valid = true;
        } else if first_word(trimmed_line) == "}" {
            is_valid = true;
            if trimmed_line.trim_start_matches("}").trim().starts_with("else") {
                if !expected_else.contains(&i) {
                    let conditional_end: usize = get_multiline_expression_end2(file.clone(), i, 1, 2);
                    skip_lines.extend(i..conditional_end);
                }
            }
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

fn get_multiline_expression_end2(file: Vec<String>, index: usize, closed_count: usize, open_count: usize) -> usize {
    let mut closed_braces_count: usize = closed_count;
    let mut open_braces_count: usize = open_count;
    let mut final_index: usize = 0;
    // Looping over each line
    for (line_index, line) in file[index..].iter().enumerate() {
        // Looping over each character in the line
        let mut in_str = false;
        for (i, c) in line.chars().enumerate() {
            if c == '\'' {
                in_str = !in_str;
            }
            if c == '}' && !in_str && line_index != 1 {
                closed_braces_count += 1;
            }
            if c == '{' && !in_str {
                open_braces_count += 1;
            }
            if closed_braces_count == open_braces_count && open_braces_count > 0 {
                final_index = line_index+index-1;
                break;
            }
        }
    }
    final_index
}

pub fn get_statement_value(file_name: &str, expression: &str) -> Value {
    let supposed_value_type: String = variables::variable_assignment::get_supposed_expression_value_type(file_name, &expression);
    let expression_value_parts: Vec<&str> = variables::variable_assignment::get_expression_value_parts(expression);
    let evaluated_value: Value = evaluate_expression_value(file_name, expression_value_parts, &supposed_value_type);

    evaluated_value
}
