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
    pub value: String,
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
    static ref GLOBAL_TREE: Mutex<HashMap<String, Value>> = Mutex::new(HashMap::new());
}
 
fn main() {
    let cwd = cwd::get_cwd().unwrap();
    let file_path = file_handler::get_file_path(cwd.to_str().unwrap());
    let file_content = file_reader::read_file(&file_path);
    add_to_global_tree("x".to_string(), Value::Int(42));
    parser::parse(file_content);
}

pub fn add_to_global_tree(key: String, value: Value) {
    let mut tree = GLOBAL_TREE.lock().unwrap();
    tree.insert(key, value);
}
