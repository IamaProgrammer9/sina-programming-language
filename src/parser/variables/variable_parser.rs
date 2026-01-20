use crate::{Value, add_to_global_tree, Variable, GLOBAL_TREE, variable_exists, get_variable};
use crate::parser::get_statement_value;
use crate::parser::variables::validators;

/// Parses & evaluates a variable declaration expression.
///
/// Only supports variable declaration expressions and must not
/// be used for reassignment expressions.
///
/// # Parameters
/// - `file_name`: The full path of the currently parsed file.
/// - `expr`: The variable declaration expression.
/// # Returns
/// Does not return any value since it automatically evaluates the expression.
/// # Panics
/// Panics if the expression represents a reassignment (e.g. `x = 15;`)
pub fn parse_variable_expression(file_name: &str, expr: &str) {
    let variable_name = get_var_name_from_expression(file_name, expr);
    let variable_value = get_var_value_from_expression(expr);
    let variable_type = get_var_type_from_expression(expr);
    let is_const = validators::is_constant(expr);
    let value: Value;

    value = get_statement_value(file_name, &variable_value);

    let variable = Variable {
        name: variable_name,
        value_type: variable_type,
        constant: is_const,
        value,
    };

    add_to_global_tree(file_name, &variable);
}

/// Extracts the variable identifier from a variable declaration expression.
///
/// This function only supports first-time variable declarations and
/// must not be used for reassignment expressions.
///
/// # Parameters
/// - `file_name`: The full path of the currently parsed file.
/// - `expr`: The variable declaration expression.
///
/// # Returns
/// The name of the declared variable.
///
/// # Panics
/// Panics if the expression represents a reassignment (e.g. `x = 15`).
///
/// # Examples
/// ```
/// let name: str = "Mohamed";
/// assert_eq!(get_var_name_from_expression("main.sina", "let name: str = \"Mohamed\";"), "name");
/// ```
fn get_var_name_from_expression(file_name: &str, expr: &str) -> String {

    let mut seperator_index: usize = 0;
    for (i, c) in expr.char_indices() {
        if c == ':' {
            seperator_index = i;
            break;
        }
    }

    let var_name = expr[..seperator_index]
        .split_whitespace()
        .nth(1)
        .unwrap_or_else(|| {
            eprintln!("Invalid variable declaration");
            std::process::exit(1);
        });

    if variable_exists(file_name, var_name) {
        eprintln!("Variable {} already exists", var_name);
        std::process::exit(1);
    }

    var_name.trim().to_string()
}

/// Extracts the variable type (int, str, ...) from a variable declaration expression.
///
/// This function only supports first-time variable declarations and
/// must not be used for reassignment expressions.
///
/// # Parameters
/// - `file_name`: The full path of the currently parsed file.
/// - `expr`: The variable declaration expression.
///
/// # Returns
/// The type of the declared variable.
///
/// # Panics
/// Panics if the expression represents a reassignment (e.g. `x = 15`).
///
/// # Examples
/// ```
/// let name: str = "Mohamed";
/// assert_eq!(get_var_name_from_expression("main.sina", "let name: str = \"Mohamed\";"), "str");
/// ```
fn get_var_type_from_expression(expr: &str) -> String {
    let mut value_type_start: usize = 0;
    let mut value_type_end: usize = 0;
    for (i, c) in expr.chars().enumerate() {
        if c == ':' && value_type_start == 0 {
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

/// Extracts the variable value (as a String) from a variable declaration expression.
///
/// This function only supports first-time variable declarations and
/// must not be used for reassignment expressions.
///
/// # Parameters
/// - `file_name`: The full path of the currently parsed file.
/// - `expr`: The variable declaration expression.
///
/// # Returns
/// The literal value (as String) of the declared variable.
///
/// # Panics
/// Panics if the expression represents a reassignment (e.g. `x = 15`).
///
/// # Examples
/// ```
/// let name: str = "Mohamed";
/// assert_eq!(get_var_name_from_expression("main.sina", "let name: str = \"Mohamed\";"), "'Mohamed'");
/// ```
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

/// Creates a `Value` from a direct literal representation.
///
/// This function converts a literal value (such as `11`, `'Omar'`, or `true`)
/// into a `Value` of the specified type. It must only be used for direct
/// (non-expression) values.
///
/// # Parameters
/// - `value`: The literal value as a string.
/// - `value_type`: The expected type of the value.
///
/// # Panics
/// Panics if the value does not match the specified value type.
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

