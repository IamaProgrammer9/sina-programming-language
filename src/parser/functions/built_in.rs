
pub fn sina_print(args: Vec<String>) {
    let mut cleaned_args: Vec<String> = Vec::new();
    for (i, c) in args.iter().enumerate() {
        cleaned_args.push(c.trim_start_matches("'").trim_end_matches("'").to_string());
    }
    println!("{}", cleaned_args.join(""));
}

fn trim_quotes(strings: &mut Vec<String>) {
    for s in strings.iter_mut() {
        *s = s.trim_matches('\'').to_string(); // remove ' from start and end
    }
}
