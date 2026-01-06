pub mod cwd;
pub mod file_handler;
pub mod file_reader;
pub mod parser;

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug, Clone)]
pub enum Value {
    Int(i32),
    Float(f64),
    Text(String),
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub value_type: String,
    pub value: Value,
    pub constant: bool,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub variables: Vec<Variable>,
}

#[derive(Debug, Clone)]
pub struct File {
    pub name: String,
    pub variables: Vec<Variable>,  // file-level variables
    pub functions: Vec<Function>,  // functions inside the file
}

lazy_static! {
    static ref GLOBAL_TREE: Mutex<HashMap<String, File>> = Mutex::new(HashMap::new());
}

fn main() {
    let cwd = cwd::get_cwd().unwrap();
    let file_path = file_handler::get_file_path(cwd.to_str().unwrap());
    let file_content = file_reader::read_file(&file_path);
    let file = File {
        name: file_path.clone(),
        variables: Vec::new(),
        functions: Vec::new(),
    };
    GLOBAL_TREE.lock().unwrap().insert(file_path.clone(), file);
    // add_to_global_tree("x".to_string(), Value::Int(42));
    parser::parse(&file_path, file_content);
}

pub fn add_to_global_tree(file_name: &str, variable: &Variable) {
    let mut tree = GLOBAL_TREE.lock().unwrap();
    if let Some(file) = tree.get_mut(file_name) {
        file.variables.push(variable.clone());
    }
}

pub fn variable_exists(file_name: &str, variable_name: &str) -> bool {
    let mut tree = GLOBAL_TREE.lock().unwrap();
    if let Some(file) = tree.get(file_name) {
        for c in file.variables.iter() {
            if c.name == variable_name {
                return true;
            }
        }
        false
    } else {
        false
    }
}

pub fn get_variable(file_name: &str, variable_name: &str) -> Option<Variable> {
    let mut tree = GLOBAL_TREE.lock().unwrap();
    if let Some(file) = tree.get(file_name) {
        file.variables.iter().find(|v| v.name == variable_name).cloned()
    } else {
        return None;
    }
}
