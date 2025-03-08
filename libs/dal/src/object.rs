use std::collections::HashMap;

/// Represents a Dal Object
pub enum Object {
    Bool(bool),
    Bytevector(Vec<u8>),
    Char(char),
    Eof,
    Null,
    Number(String),
    Pair(Box<Object>, Box<Object>),
    String(String),
    Symbol(String),
    Vector(Vec<Object>),
}

#[derive(Debug)]
pub enum Atom {
    Bool(bool),
    Bytevector(Vec<u8>),
    Char(char),
    Eof,
    Null,
    Number(String),
    String(String),
    Symbol(String),
}


#[derive(Debug)]
pub enum Sexp {
    Atom(Atom),
    Pair(Box<Sexp>, Box<Sexp>),
}

impl Sexp {
    pub fn eval(&self, _: &mut HashMap<String, Object>) -> Result<Object, Box<dyn std::error::Error + Send + Sync>> {
        Ok(Object::Number("42".to_string()))
    }
}

