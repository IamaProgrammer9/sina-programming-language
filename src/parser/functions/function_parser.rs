use crate::parser::functions::function_call::get_call_arguments;

pub fn parse_function(file_name: &str, function: Vec<String>) {
    // Getting the function name
    let function_name = parse_function_name(&function[0]);
}

pub fn parse_function_name(first_line: &str) -> String {
    let mut function_name: String = "".to_string();
    let mut function_name_end: usize = 0;
    for (i, c) in first_line.trim_start_matches("fn ").chars().enumerate() {
        if c == '(' {
            function_name = first_line.trim_start_matches("fn ")[0..i].to_string();
            break;
        }
    }
    if function_name == "" {
        eprintln!("Cannot create a nameless function");
        std::process::exit(1);
    }
    function_name
}
