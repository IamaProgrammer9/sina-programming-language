use crate::{Value, add_to_global_tree, Variable, GLOBAL_TREE};

pub fn parse_variable_expression(file_name: &str, expr: &str) {
    let variable_name = expr.split_whitespace().nth(1).unwrap().replace(":", "");
    let variable_type = expr.split_whitespace().nth(2).unwrap().replace(";", "");
    let variable_value = expr.split_whitespace().nth(4).unwrap().replace(";", "");
    let is_const = is_constant(expr);
    let value: Value;

    value = get_value(&variable_value, &variable_type);

    let variable = Variable {
        name: variable_name,
        value_type: variable_type,
        constant: is_const,
        value,
    };

    add_to_global_tree(file_name, variable);

    println!("TREE: {:?}", GLOBAL_TREE.lock().unwrap());
}

fn is_constant(expr: &str) -> bool {
    let state = expr.split_whitespace().nth(0).unwrap();
    if state == "const" {
        return true;
    } else {
        return false;
    }
}

// fn get_variable_name(expr: &str) -> &str {}

fn get_value(value: &str, value_type: &str) -> Value {
    if value_type == "int" {
        if !valid_int(value) {
            eprintln!("Value type does not match with int: {}", value);;
            std::process::exit(1);
        }
        Value::Int(value.parse::<i32>().unwrap())
    } else if value_type == "str" {
        if !valid_str(value) {
            eprintln!("Value type does not match with str: {}", value);
            std::process::exit(1);
        }
        let trimmed_value = &value[1..value.len() - 1];
        Value::Text(trimmed_value.parse::<String>().unwrap())
    } else if value_type == "float" {
        if !valid_float(value) {
            eprintln!("Value type does not match with float: {}", value);
            std::process::exit(1);
        }
        Value::Float(value.parse::<f64>().unwrap())
    } else {
        eprint!("Unsupported value type: {}", value_type);
        std::process::exit(1);
    }
}

fn valid_int(value: &str) -> bool {
    value.parse::<i32>().is_ok()
}

fn valid_str(value: &str) -> bool {
    if value.ends_with("'") & value.starts_with("'") {
        let trimmed_value = value.trim_start_matches("'").trim_end_matches("'");
        if trimmed_value.parse::<String>().is_ok() {
            true
        } else {
            false
        }
    } else {
        false
    }
}

fn valid_float(value: &str) -> bool {
    value.parse::<f64>().is_ok()
}
