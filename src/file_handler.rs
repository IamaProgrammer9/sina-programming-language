use std::env;

pub fn get_file_path(cwd: &str) -> String {
    let args: Vec<String> = env::args().collect();
    if !args.len() == 2 {
        eprintln!("Usage: {} <file_name>", args[0]);
        std::process::exit(1);
    }
    let file_name: &str = &args[1];
    let file_path = format!("{}/{}", cwd, file_name);
    file_path
}