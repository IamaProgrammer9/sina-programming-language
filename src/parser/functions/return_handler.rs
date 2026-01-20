use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum ReturnType {
    Int,
    Float,
    Bool,
    String,
    Void,
}

#[derive(Debug)]
pub struct ReturnTypeContext {
    pub functions: HashMap<String, ReturnType>,
}

impl ReturnTypeContext {
    pub fn new() -> Self {
        let mut functions = HashMap::new();

        // prebuilt functions
        functions.insert("print".to_string(), ReturnType::Void);
        functions.insert("input".to_string(), ReturnType::String);
        functions.insert("int_input".to_string(), ReturnType::Int);

        Self { functions }
    }

    pub fn register_fn(&mut self, name: &str, return_type: ReturnType) {
        self.functions.insert(name.to_string(), return_type);
    }

    pub fn get_return_type(&self, name: &str) -> Option<&ReturnType> {
        self.functions.get(name)
    }
}

pub fn option_return_type_to_str(rt: Option<&ReturnType>) -> &str {
    match rt {
        Some(ReturnType::Int) => "int",
        Some(ReturnType::Float) => "float",
        Some(ReturnType::Bool) => "bool",
        Some(ReturnType::String) => "str",
        Some(ReturnType::Void) => "null",
        None => "unknown",
    }
}

