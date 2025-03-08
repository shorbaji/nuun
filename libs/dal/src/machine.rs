use crate::error::DalError;
use crate::lexer::{DLexer, Token};
use crate::object::{Atom, Object, Sexp};
use crate::parser::Parser;
use logos::Logos;
use uuid::Uuid;
use std::collections::HashMap;

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
        let mut _parser = Parser::new(code);
        Ok(Object::Null)
    }
}


