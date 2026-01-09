use crate::{Value, add_to_global_tree, Variable, GLOBAL_TREE, variable_exists, get_variable};
use crate::parser::variables::validators;

pub fn parse_variable_expression(file_name: &str, expr: &str) {
    let variable_name = get_var_name_from_expression(file_name, expr);
    let variable_type = get_var_type_from_expression(expr);
    let variable_value = get_var_value_from_expression(expr);
    let is_const = validators::is_constant(expr);
    let value: Value;

    value = create_value(&variable_value, &variable_type);

    let variable = Variable {
        name: variable_name,
        value_type: variable_type,
        constant: is_const,
        value,
    };

    add_to_global_tree(file_name, &variable);
}

fn get_var_name_from_expression(file_name: &str, expr: &str) -> String {
    let mut var_name = expr
        .split_whitespace()
        .nth(1)
        .unwrap()
        .replace(";", "");

    if variable_exists(file_name, &var_name) {
        eprintln!("Variable {} already exists", var_name);
        std::process::exit(1);
    }

    if let Some(i) = var_name.find(':') {
        var_name.truncate(i);
    }

    var_name
}

fn get_var_type_from_expression(expr: &str) -> String {
    let mut value_type_start: usize = 0;
    let mut value_type_end: usize = 0;
    for (i, c) in expr.chars().enumerate() {
        if c == ':' {
            value_type_start = i+1;
        }
        if c == '=' {
            if value_type_start == 0 {
                eprintln!("Variable type not defined");
                std::process::exit(1);
            }
            value_type_end = i;
        }
    }
    expr[value_type_start..value_type_end].trim().to_string()
}

fn get_var_value_from_expression(expr: &str) -> String {
    let mut value_start: usize = 0;
    for (i, c) in expr.chars().enumerate() {
        if c == '=' {
            value_start = i+1;
            break;
        }
    }
    expr[value_start..].trim().trim_end_matches(";").to_string()
}

// fn get_variable_name(expr: &str) -> &str {}

pub fn create_value(value: &str, value_type: &str) -> Value {
    if value_type == "int" {
        if !validators::valid_int(value) {
            eprintln!("Value type does not match with int: {}", value);;
            std::process::exit(1);
        }
        Value::Int(value.parse::<i32>().unwrap())
    } else if value_type == "str" {
        if !validators::valid_str(value) {
            eprintln!("Value type does not match with str: {}", value);
            std::process::exit(1);
        }
        let trimmed_value = &value[1..value.len() - 1];
        Value::Text(trimmed_value.parse::<String>().unwrap())
    } else if value_type == "float" {
        if !validators::valid_float(value) {
            eprintln!("Value type does not match with float: {}", value);
            std::process::exit(1);
        }
        Value::Float(value.parse::<f64>().unwrap())
    } else {
        eprint!("Unsupported value type: {}", value_type);
        std::process::exit(1);
    }
}

