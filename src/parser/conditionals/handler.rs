use crate::parser::variables::variable_assignment::get_value;
use crate::parser::variables::variable_assignment::evaluate_expression_value;

pub fn evaluate_conditional(file_name: &str, conditional: Vec<&str>) {
    let first_line =  conditional[0].to_string();
    let conditional_statement = get_conditional_statement(&first_line);
    let condition_type = get_conditional_type(conditional_statement);
    println!("{}", condition_type);
    // let evaluated_expression = evaluate_expression_value(conditional_statement);
}

fn get_conditional_statement(line: &str) -> &str {
    line.trim_start_matches("if").trim_end_matches(" {")
}

fn get_conditional_type(statement: &str) -> &str {
    let mut in_str: bool = false;
    let mut conditional_type: &str = "";
    for (i, c) in statement.chars().enumerate() {
        if c == '\'' {
            in_str = !in_str;
        }
        if c == '=' && !in_str {
            if statement.chars().nth(i + 1).unwrap() == '=' {
                conditional_type = "equation";
                break;
            } else {
                eprintln!("Invalid equal sign at conditional.");
                std::process::exit(1);
            }
        }
        if c == '!' && !in_str {
            if statement.chars().nth(i + 1).unwrap() == '=' {
                conditional_type = "inequality";
                break;
            }
        }
        if c == '<' {
            if statement.chars().nth(i + 1).unwrap() == '=' {
                conditional_type = "less_than_or_equal";
            } else {
                conditional_type = "less_than";
            }
            break;
        }
        if c == '>' {
            if statement.chars().nth(i + 1).unwrap() == '=' {
                conditional_type = "greater_than_or_equal";
            } else {
                conditional_type = "greater_than";
            }
            break;
        }
    }
    conditional_type
}
