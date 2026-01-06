use crate::{Value, add_to_global_tree, Variable, GLOBAL_TREE, variable_exists, get_variable};
use crate::parser::variables::validators;

pub fn parse_variable_expression(file_name: &str, expr: &str) {
    let variable_name = expr.split_whitespace().nth(1).unwrap().replace(":", "");
    let variable_type = expr.split_whitespace().nth(2).unwrap().replace(";", "");
    let variable_value = expr.split_whitespace().nth(4).unwrap().replace(";", "");
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

// fn get_variable_name(expr: &str) -> &str {}

fn create_value(value: &str, value_type: &str) -> Value {
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

