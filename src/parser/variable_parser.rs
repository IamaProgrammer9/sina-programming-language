use crate::{Value, add_to_global_tree};
use crate::GLOBAL_TREE;

pub fn parse_variable_expression(expr: &str) {
    let variable_name = expr.split_whitespace().nth(1).unwrap().replace(":", "");
    let variable_type = expr.split_whitespace().nth(2).unwrap().replace(";", "");
    let variable_value = expr.split_whitespace().nth(4).unwrap().replace(";", "");
    let mut value: Value;

    if variable_type == "int" {
        value = Value::Int(variable_value.parse::<i32>().unwrap());
    } else if variable_type == "float" {
        value = Value::Float(variable_value.parse::<f64>().unwrap());
    } else if variable_type == "str" {
        value = Value::Text(variable_value.replace("\"", ""));
    } else {
        eprintln!("Unsupported variable type: {}", variable_type);
        std::process::exit(1);
    }

    add_to_global_tree(variable_name, value);

    println!("Variables: {:?}", GLOBAL_TREE.lock().unwrap());
}