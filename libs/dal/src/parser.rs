use crate::lexer::{DLexer, Token};
use crate::object::{Atom, Sexp};
use crate::error::DalError;

pub struct Parser {
    tokens: DLexer,
}

impl Parser {
    pub fn new(code: &str) -> Self {

        Self {
            tokens: DLexer::new(code),
        }
    }

    fn peek(&mut self) -> Option<Result<Token, ()>> {
        self.tokens.peek()
    }

    fn expect_token(&mut self, expected: Token) -> Result<Token, DalError> {
        self.peek()
        .ok_or(DalError::ParserError("expected something".to_string()))
        .and_then(|result|
            result
            .map_err(|_| DalError::LexerError)
            .and_then(|token| {
                if token == expected {
                    Ok(token)
                } else {
                    Err(DalError::ParserError(format!("expected {:?}", expected)))
                }
            })
        )
        .inspect(|_| {self.tokens.next(); })
    }

    fn paren_left(&mut self) -> Result<(), DalError> {
        self.expect_token(Token::ParenLeft)
        .map(|_| ())
    }

    fn paren_right(&mut self) -> Result<(), DalError> {
        self.expect_token(Token::ParenRight)
        .map(|_| ())
    }


    fn true_(&mut self) -> Result<Sexp, DalError> {
        self.expect_token(Token::Boolean(true))
        .map(|_| Sexp::Atom(Atom::Bool(true)))
    }

    fn false_(&mut self) -> Result<Sexp, DalError> {
        self.expect_token(Token::Boolean(false))
        .map(|_| Sexp::Atom(Atom::Bool(false)))
    }

    fn boolean(&mut self) -> Result<Sexp, DalError> {
        self.true_()
        .or_else(|_| self.false_())
    }


    fn get(&mut self, expected: Token) -> Result<(), DalError> {
        self.tokens.peek()
        .map(|result|
            result
            .map_err(|e| DalError::LexerError)
            .and_then(|token| {
                if token == expected {
                    Ok(())
                } else {
                    Err(DalError::ParserError(format!("expected {:?}", expected)))
                }
            })
        )
        .unwrap_or(Err(DalError::ParserError("expected something".to_string())))
    }

    fn one_or_more<F, T>(&mut self, mut f: F) -> Result<(), DalError>
    where
        F: FnMut(&mut Self) -> Option<Result<T, DalError>>
    {
        f(self).ok_or(DalError::ParserError("expected one or more".to_string()))?;

        while let Some(result) = f(self) {
            match result {
                Ok(_) => {},
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }

    fn sexp(&mut self) -> Result<Sexp, DalError> {
        unimplemented!("sexp")
    }
}

impl std::iter::Iterator for Parser {
    type Item = Result<Sexp, DalError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokens
        .peek()
        .and_then(|_|
            Some(self.sexp()))
        
    }
}

