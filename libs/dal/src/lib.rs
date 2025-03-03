//! # Dal
//!
//! An interpreter for the Dal languagea
//!
//! ## Overview
//!
//! Dal is a simple Lisp-like language. This module defines an interpreter for the Dal language.
//! The main components of the interpreter are:
//! - Machine: the interpreter itself
//! - Object: the data types of the language
//!

mod lexer;

use logos::Logos;
use std::collections::HashMap;
use uuid::Uuid;

/// Represents a Dal Object
pub enum Object {
    Bool(bool),
    Bytevector(Vec<u8>),
    Char(char),
    Eof,
    Null,
    Number(f64),
    Pair(Box<Object>, Box<Object>),
    String(String),
    Symbol(String),
    Vector(Vec<Object>),
}

struct Parser {
}

impl Parser {
    pub fn new(code: &str) -> Self {
        Self {
            lexer: lexer::Dlexer(code),
        }
    }

    fn cdr(&mut self) -> Result<Object, Box<dyn std::error::Error + Send + Sync>> {
        // we need to peek at the next token
        let token = self.lexer.next();
        match token {
            Some(Token::Dot) => {
                sexp = self.next()?;
                let token = self.lexer.next();
                match token {
                    Some(Token::ParenClose) => Ok(sexp),
                    _ => Err("expected )".into()),
                }
            },
            Some(Token::ParenClose) => Ok(Object::Null),
            _ => Err("expected . or )".into()),
        }
    }
}

impl std::iter::Iterator for Parser {
    type Item = Result<Sexp, Box<dyn std::error::Error + Send + Sync>>;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.lexer.peek();
        match token {
            Some(Token::ParenClose) => Some(Err("unexpected )".into())),
            Some(Token::Number(n)) => Some(Ok(Sexp::Atom(Atom::Number(n)))),
            Some(Token::Symbol(s)) => Some(Ok(Sexp::Atom(Atom::Symbol(s)))),
            None => None,
            _ => unimplemented!("unexpected token"),
        }

    }
}
pub struct Machine {
    id: Uuid,
    global_env: HashMap<String, Object>,
}

impl std::fmt::Debug for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Machine id: {:?}", self.id)
    }
}

impl Default for Machine {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Machine id: {:?}", self.id)
    }
}

impl Machine {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            global_env: HashMap::new(),
        }
    }

    // fn improper_list(&self, tokens: &mut Vec<Token>) -> Result<Sexp, Box<dyn std::error::Error + Send + Sync>> {
    //     let cdr = self.sexp(tokens)?;
    //     let token = tokens.remove(0);
    //     match token {
    //         Token::ParenClose => Ok(cdr),
    //         _ => Err("expected )".into()),
    //     }
    // }

    // fn cdr(&self, tokens: &mut Vec<Token>) -> Result<Sexp, Box<dyn std::error::Error + Send + Sync>> {
    //     let token = tokens[0].clone();
    //     match token {
    //         Token::Dot => self.improper_list(tokens),
    //         Token::ParenClose => {
    //             tokens.remove(0);
    //             Ok(Sexp::Atom(Atom::Null))
    //         },
    //         _ => {
    //             let car = self.sexp(tokens)?;
    //             let cdr = self.cdr(tokens)?;
    //             Ok(Sexp::Pair(Box::new(car), Box::new(cdr)))
    //         }
    //     }
    // }

    // fn sexp(&self, tokens: &mut Vec<Token>) -> Result<Sexp, Box<dyn std::error::Error + Send + Sync>> {
    //     let token = tokens.remove(0);
    //     match token {
    //         Token::Number(n) => Ok(Sexp::Atom(Atom::Number(n))),
    //         Token::Symbol(s) => Ok(Sexp::Atom(Atom::Symbol(s))),
    //         Token::ParenClose => Err("unexpected )".into()),
    //         Token::ParenOpen => {
    //             let car = self.sexp(tokens)?;
    //             let cdr = self.cdr(tokens)?;
    //             Ok(Sexp::Pair(Box::new(car), Box::new(cdr)))
    //         }
    //         _ => Err("unexpected token".into()),
    //     }
    // }

    // fn parse(&self, tokens: &mut Vec<Token>) -> Result<Vec<Sexp>, Box<dyn std::error::Error + Send + Sync>> {
    //     let mut program: Vec<Sexp> = vec![];

    //     while !tokens.is_empty() {
    //         program.push(self.sexp(tokens)?);
    //     }

    //     Ok(program)
    // }

    // async fn interpret(&mut self, ast: &Vec<Sexp>) -> Result<Object, Box<dyn std::error::Error + Send + Sync>> {
    //     let mut result = Object::Null;

    //     for sexp in ast
    //     {
    //         result = sexp.eval(&mut self.global_env)?;
    //     }

    //     Ok(result)
    // }

    pub async fn eval(&mut self, code: &str) -> Result<Object, Box<dyn std::error::Error + Send + Sync>> {
        let mut lex = Token::lexer(code);

        let sexps = self.parse(&mut lex)?;
        // let result = self.interpret(&sexps).await?;
        Ok(Object::Null)
    }
}


#[derive(Debug)]
enum Atom {
    Bool(bool),
    Bytevector(Vec<u8>),
    Char(char),
    Eof,
    Null,
    Number(f64),
    String(String),
    Symbol(String),
}


#[derive(Debug)]
enum Sexp {
    Atom(Atom),
    Pair(Box<Sexp>, Box<Sexp>),
}

impl Sexp {
    pub fn eval(&self, _: &mut HashMap<String, Object>) -> Result<Object, Box<dyn std::error::Error + Send + Sync>> {
        Ok(Object::Number(42.0))
    }
}

// a Dal Machine
