pub fn get_function_name(call: &str) -> &str {
    if let Some(index) = call.find('(') {
        return &call[..index];
    } else {
        eprintln!("Invalid function call");
        std::process::exit(1);
    }
}

pub fn get_call_index(call: &str) -> usize {
    if let Some(index) = call.find('(') {
        index
    } else {
        eprintln!("Invalid function call");
        std::process::exit(1);
    }
}
