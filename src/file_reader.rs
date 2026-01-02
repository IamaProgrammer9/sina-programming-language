use std::fs::{self, read_to_string};
use std::path::Path;

pub fn read_file(file_path: &str) -> Vec<String> {
    let path = Path::new(file_path);
    if let Some(ext) = path.extension() {
        if ext != "sina" {
            eprint!("only .sina files are supported");
            std::process::exit(1);
        }
    } else {
        eprintln!("File has no extension");
        std::process::exit(1);
    };

    let mut result = Vec::new();
    for line in read_to_string(path).unwrap().lines() {
        result.push(line.to_string());
    }

    result
}
