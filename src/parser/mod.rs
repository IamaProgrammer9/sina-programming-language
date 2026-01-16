use crate::add_to_global_tree;
mod variables;
mod functions;

pub fn parse(file_name: &str, file: Vec<String>) {
    for (i, line) in file.iter().enumerate() {
        let trimmed_line: &str = line.trim();
        let mut is_valid = false;

        // Skip empty lines and comments
        if trimmed_line.is_empty() || trimmed_line.starts_with('#') {
            continue;
        }

        // Variable detection
        if first_word(trimmed_line) == "let" || first_word(trimmed_line) == "const" {
            variables::variable_parser::parse_variable_expression(file_name, trimmed_line);
            is_valid = true;
        } else if first_word(trimmed_line) == "fn" {
            let function_end: usize = get_multiline_expression_end(file.clone(), i);
            is_valid = true;
            functions::function_parser::parse_function(file_name, file[i..function_end].to_vec());
        } else if first_word(trimmed_line) == "}" {
            is_valid = true;
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
