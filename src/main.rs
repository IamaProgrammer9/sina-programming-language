pub mod cwd;
pub mod file_handler;
pub mod file_reader;
pub mod parser;

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i32),
    Float(f64),
    Text(String),
    Bool(bool),
}

impl Value {
    pub fn as_int(&self) -> Option<i32> {
        if let Value::Int(n) = self {
            Some(*n)
        } else {
            None
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        if let Value::Float(n) = self {
            Some(*n)
        } else {
            None
        }
    }

    pub fn as_text(&self) -> Option<&str> {
        if let Value::Text(s) = self {
            Some(s)
        } else {
            None
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        if let Value::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
}


#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub value_type: String,
    pub value: Value,
    pub constant: bool,
}

impl Variable {
    // Return the value as a string
    pub fn value_as_string(&self) -> String {
        match &self.value {
            Value::Int(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::Text(s) => s.clone(),
            Value::Bool(b) => b.to_string(),
        }
    }
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
    // Registering built-in functions to work properly
    parser::functions::built_in::register_built_in_functions();
    // Parsing the main file
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

pub fn set_variable_value(
    file_name: &str,
    variable_name: &str,
    new_value: Value,
) {
    let mut tree = GLOBAL_TREE.lock().unwrap();

    let file = tree
        .get_mut(file_name)
        .expect("File not found");

    let var = file
        .variables
        .iter_mut()
        .find(|v| v.name == variable_name)
        .expect("Variable not found");

    var.value = new_value;
}

pub fn get_variable_type(file_name: &str, variable_name: &str) -> String {
    let mut tree = GLOBAL_TREE.lock().unwrap();
    let file = tree
        .get_mut(file_name)
        .expect("File not found");

    let var = file
        .variables
        .iter_mut()
        .find(|v| v.name == variable_name)
        .expect("Variable not found");

    var.value_type.clone()
}

pub fn is_constant(file_name: &str, variable_name: &str) -> bool {
    let mut tree = GLOBAL_TREE.lock().unwrap();
    let file = tree
        .get_mut(file_name)
        .expect("File not found");

    let var = file
        .variables
        .iter_mut()
        .find(|v| v.name == variable_name)
        .expect("Variable not found");

    var.constant
}
