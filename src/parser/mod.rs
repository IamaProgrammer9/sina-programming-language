use crate::add_to_global_tree;
mod variables;
mod functions;

pub fn parse(file_name: &str, file: Vec<String>) {
    for line in file {
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


