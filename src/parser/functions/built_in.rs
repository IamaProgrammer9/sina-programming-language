use std::io;

pub fn sina_print(args: Vec<String>) {
    let mut cleaned_args: Vec<String> = Vec::new();
    for (i, c) in args.iter().enumerate() {
        cleaned_args.push(c.trim_start_matches("'").trim_end_matches("'").to_string());
    }
    println!("{}", cleaned_args.join(""));
}

pub fn sina_input(message: &str) -> String {
    print!("{}", message);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim().to_string();
    input
}
