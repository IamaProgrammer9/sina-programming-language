use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref GLOBAL_FUNCTION_RETURN_TYPES: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

/// Registers the return type of a specific function in the `GLOBAL_FUNCTION_RETURN_TYPES` Mutex.
///
/// # Arguments
/// - `function_name`: Name of the function as a `&str`. For user-defined
///   functions, this should include the full path of the currently parsed
///   file followed by a `/`. Built-in functions may use their plain name.
/// - `return_type`: &str of the returned type of the function (str, int, ...).
/// # Returns
/// None.
pub fn register_function_return_type(function_name: &str, return_type: &str) {
    GLOBAL_FUNCTION_RETURN_TYPES
        .lock()
        .unwrap()
        .insert(function_name.to_string(), return_type.to_string());
}

/// Gets the return type of a registered function.
///
/// # Arguments
/// - `function_name`: Name of the function as a `&str`. For user-defined
///   functions, this should include the full path of the currently parsed
///   file followed by a `/`. Built-in functions may use their plain name.
///
/// # Returns
/// `Some(String)` containing the function's return type if the function is
/// registered, or `None` if no such function exists.
pub fn get_function_return_type(function_name: &str) -> Option<String> {
    GLOBAL_FUNCTION_RETURN_TYPES
        .lock()
        .unwrap()
        .get(function_name)
        .cloned()
}
