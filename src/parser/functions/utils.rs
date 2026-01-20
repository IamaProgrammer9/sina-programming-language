/// Returns the index that separates a function name from its arguments when it's called.
pub fn get_call_index(call: &str) -> usize {
    if let Some(index) = call.find('(') {
        index
    } else {
        eprintln!("Invalid function call");
        std::process::exit(1);
    }
}