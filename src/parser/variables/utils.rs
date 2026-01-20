/// Extracts the function name of a function call expression (e.g).
///
/// # Arguments
/// - `func_call`: the function call expression.
/// # Returns
/// String or None if it's an invalid expression.
/// # Examples
/// `get_user(id: 3)` -> `get_user`.
/// # Notes
/// This function does not check whether the function exists in the parsed file or not.
pub fn extract_func_name_from_call(func_call: &str) -> Option<String> {
    for (i, c) in func_call.char_indices() {
        if c == '(' {
            return Some(func_call[..i].to_string());
        }
    }
    None
}